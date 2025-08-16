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

const MAX_ITER: usize = 1000;

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

fn calc_color(count: usize) -> [f32; 3] {
    const IN_COLOR: [f32; 3] = [1., 1., 1.];
    const OUT_COLOR: [f32; 3] = [0., 0., 0.];
    if count == MAX_ITER {
        IN_COLOR
    } else {
        let ratio = dbg!(count as f32 / MAX_ITER as f32);

        [
            (IN_COLOR[0] - OUT_COLOR[0]) * ratio + OUT_COLOR[0],
            (IN_COLOR[1] - OUT_COLOR[1]) * ratio + OUT_COLOR[1],
            (IN_COLOR[2] - OUT_COLOR[2]) * ratio + OUT_COLOR[2],
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
    fn range(&self) -> T {
        self.max - self.min
    }

    fn map(&self, value: T) -> T {
        self.min + value * self.range()
    }
}
