use std::ops::{Add, Mul, Sub};

use image::ImageBuffer;
use num::{Complex, complex::ComplexFloat};

const X_AXIS: Axis<f64> = Axis {
    max: 1.0,
    min: -2.0,
};
const Y_AXIS: Axis<f64> = Axis { max: 1., min: -1. };
const RATIO: f64 = (X_AXIS.max - X_AXIS.min) / (Y_AXIS.max - Y_AXIS.min);

const WIDTH: u32 = 10000;
const HEIGHT: u32 = (WIDTH as f64 / RATIO) as u32;

const MAX_ITER: u16 = 100;

fn main() {
    let i = std::sync::Arc::new(std::sync::Mutex::new(0));

    std::thread::spawn({
        let i = i.clone();
        move || loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let count = *i.lock().unwrap();
            let total = WIDTH * HEIGHT;
            let percent = (count as f64 / total as f64) * 100.0;
            print!("Progress: {percent:.2}% ({count}/{total})");
            if count >= total {
                break;
            }
            print!("\r");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    });

    let image = ImageBuffer::from_par_fn(WIDTH, HEIGHT, |x, y| {
        let cx = X_AXIS.map(x as f64 / WIDTH as f64);
        let cy = Y_AXIS.map(y as f64 / HEIGHT as f64);

        let c = Complex::new(cx, cy);
        let count = iter_count(c);

        *i.lock().unwrap() += 1;

        image::Rgb(calc_color(count))
    });

    image.save("mandelbrot.png").expect("Failed to save image");
}

fn iter_count(c: Complex<f64>) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        if z.abs() > 2.0 {
            return i as usize;
        }
        z = z * z + c;
    }
    return MAX_ITER as usize;
}

fn calc_color(count: usize) -> [u16; 3] {
    if count == MAX_ITER as usize {
        [0, 0, 0]
    } else {
        let t = (count as f64 / MAX_ITER as f64).powf(0.4);
        let cycles = 5.0;
        let h = 360.0 * t * cycles % 360.0;
        let s = 1.0;
        let v = 1.0;

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r1, g1, b1) = match h as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let r = ((r1 + m) * 65535.0) as u16;
        let g = ((g1 + m) * 65535.0) as u16;
        let b = ((b1 + m) * 65535.0) as u16;
        [r, g, b]
    }
}

struct Axis<T> {
    max: T,
    min: T,
}

impl<T> Axis<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    fn range(&self) -> T {
        self.max - self.min
    }

    fn map(&self, value: T) -> T {
        self.min + value * self.range()
    }
}
