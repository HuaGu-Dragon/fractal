use std::ops::{Add, Mul, Sub};

use image::ImageBuffer;
use num::{Complex, complex::ComplexFloat};

const X_AXIS: Axis<f64> = Axis {
    max: 1.0,
    min: -2.0,
};
const Y_AXIS: Axis<f64> = Axis { max: 1., min: -1. };
const RATIO: f64 = (X_AXIS.max - X_AXIS.min) / (Y_AXIS.max - Y_AXIS.min);

const WIDTH: u32 = 800;
const HEIGHT: u32 = (WIDTH as f64 / RATIO) as u32;

const MAX_ITER: u8 = u8::MAX;

fn main() {
    let image = ImageBuffer::from_par_fn(WIDTH, HEIGHT, |x, y| {
        let cx = X_AXIS.map(x as f64 / WIDTH as f64);
        let cy = Y_AXIS.map(y as f64 / HEIGHT as f64);

        let c = Complex::new(cx, cy);
        let count = iter_count(c);

        image::Rgb(calc_color(count as u8))
    });

    image.save("mandelbrot.png").expect("Failed to save image");
}

fn iter_count(c: Complex<f64>) -> u8 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        if z.abs() > 2.0 {
            return i as u8;
        }
        z = z * z + c;
    }
    return MAX_ITER;
}

fn calc_color(ratio: u8) -> [u8; 3] {
    const IN_COLOR: [u8; 3] = [255, 255, 255];
    if ratio == MAX_ITER {
        IN_COLOR
    } else {
        [ratio, ratio, ratio]
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
