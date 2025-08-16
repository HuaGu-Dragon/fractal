use std::ops::{Add, Mul, Sub};

use image::ImageBuffer;
use num::{Complex, complex::ComplexFloat};

const X_AXIS: Axis<f64> = Axis {
    max: 1.0,
    min: -2.0,
};
const Y_AXIS: Axis<f64> = Axis { max: 1., min: -1. };
const RATIO: f64 = X_AXIS.range() / Y_AXIS.range();

const WIDTH: u32 = 800;
const HEIGHT: u32 = (WIDTH as f64 / RATIO) as u32;

const MAX_ITER: usize = 1000;

const BASE_COLOR: [u8; 3] = [200, 200, 230];
const MAX_COLOR: [u8; 3] = [44, 60, 80];

fn main() {
    let image = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        let cx = X_AXIS.map(x as f64 / WIDTH as f64);
        let cy = Y_AXIS.map(y as f64 / HEIGHT as f64);

        let c = Complex::new(cx, cy);
        let count = iter_count(c);

        image::Rgb(calc_color(count))
    });

    image.save("mandelbrot.png").expect("Failed to save image");
}

fn iter_count(c: Complex<f64>) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        if z.abs() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    return MAX_ITER;
}

fn calc_color(count: usize) -> [u8; 3] {
    if count == MAX_ITER {
        MAX_COLOR
    } else {
        let ratio = dbg!(count as f64 / MAX_ITER as f64);

        [
            ((BASE_COLOR[0] - MAX_COLOR[0]) as f64 * ratio + BASE_COLOR[0] as f64) as u8,
            ((MAX_COLOR[1] - BASE_COLOR[1]) as f64 * ratio + BASE_COLOR[1] as f64) as u8,
            ((MAX_COLOR[2] - BASE_COLOR[2]) as f64 * ratio + BASE_COLOR[2] as f64) as u8,
        ]
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
    const fn range(&self) -> T {
        self.max - self.min
    }

    fn map(&self, value: T) -> T {
        self.min + value * self.range()
    }
}
