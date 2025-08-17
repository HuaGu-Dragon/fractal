use std::{
    ops::{Add, Mul, Sub},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use clap::{Parser, Subcommand};
use image::ImageBuffer;
use num::{Complex, complex::ComplexFloat};
use rayon::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Mandelbrot {
        #[arg(short, long, default_value_t = String::from("mandelbrot"))]
        name: String,

        #[arg(short, long, default_value_t = 100)]
        iter: usize,

        #[arg(long, default_value_t = -2.0)]
        from_x: f64,

        #[arg(long, default_value_t = -1.0)]
        from_y: f64,

        #[arg(long, default_value_t = 1.0)]
        to_x: f64,

        #[arg(long, default_value_t = 1.0)]
        to_y: f64,

        #[arg(short, long, default_value_t = 400)]
        width: u32,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Mandelbrot {
            name,
            iter,
            from_x,
            from_y,
            to_x,
            to_y,
            width,
        } => {
            let i = Arc::new(AtomicUsize::new(0));

            let x_axis = Axis {
                max: to_x,
                min: from_x,
                range: to_x - from_x,
            };

            let y_axis = Axis {
                max: to_y,
                min: from_y,
                range: to_y - from_y,
            };

            let ratio = x_axis.range / y_axis.range;

            let height = (width as f64 / ratio) as u32;

            std::thread::spawn({
                let i = i.clone();
                let total = (width * height) as usize;
                move || loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    let count = i.load(Ordering::Relaxed);
                    let percent = (count as f64 / total as f64) * 100.0;
                    print!("Progress: {percent:.2}% ({count}/{total})");
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    if count >= total {
                        break;
                    }
                    print!("\r");
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                }
            });

            let color_table: Vec<[u16; 3]> = {
                (0..=iter)
                    .into_par_iter()
                    .map(|i| calc_color(iter, i as f64))
                    .collect()
            };

            let inv_w = 1.0 / width as f64;
            let inv_h = 1.0 / height as f64;
            let image = ImageBuffer::from_par_fn(width, height, |x, y| {
                let cx = x_axis.map(x as f64 * inv_w);
                let cy = y_axis.map(y as f64 * inv_h);

                let c = Complex::new(cx, cy);
                let mu = iter_smooth(iter, c);

                i.fetch_add(1, Ordering::Relaxed);

                image::Rgb(color_table[mu as usize])
            });

            let path = name + ".png";

            image.save(path).expect("Failed to save image");
        }
    }
}

#[inline(always)]
fn iter_smooth(iter: usize, c: Complex<f64>) -> f64 {
    let mut i = 0;
    let mut period = 0;

    let mut z = Complex::new(0.0, 0.0);
    let mut z_prev = z;

    while i < iter {
        if z.norm_sqr() > 4.0 {
            return i as f64 + 1.0 - (z.norm_sqr().ln() / 2.).ln() / std::f64::consts::LN_2;
        }
        z = z * z + c;
        i += 1;

        period += 1;
        if period > 20 {
            if (z - z_prev).norm_sqr() < 1e-12 {
                return iter as f64;
            }
            z_prev = z;
            period = 0;
        }
    }
    iter as f64
}

#[inline(always)]
fn calc_color(iter: usize, mu: f64) -> [u16; 3] {
    if mu >= iter as f64 {
        [0, 0, 0]
    } else {
        let t = (mu / iter as f64).powf(0.4);
        let h = 360.0 * t;
        let c = 1.0;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());

        let (r1, g1, b1) = match h as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let r = (r1 * 65535.0) as u16;
        let g = (g1 * 65535.0) as u16;
        let b = (b1 * 65535.0) as u16;
        [r, g, b]
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct Axis<T> {
    max: T,
    min: T,
    range: T,
}

impl<T> Axis<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    #[inline]
    fn map(&self, value: T) -> T {
        self.min + value * self.range
    }
}
