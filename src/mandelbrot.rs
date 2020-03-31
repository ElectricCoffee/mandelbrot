#![allow(unused)]
use super::color::{BLACK, COLORS};
const ITERATION_BOUND: usize = 50;

use num_complex::Complex64;

/// The "growth" of the mandelbrot set in iterations, see `explodes_after`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Growth {
    Stable,
    After(usize),
}

impl Growth {
    /// Converts the `Growth` to a colour depending on how many iterations it takes for it to explode
    pub fn to_rgb(self) -> [u8; 3] {
        use Growth::*;
        match self {
            Stable => BLACK,
            After(n) => COLORS[(n / 4) % 12],
        }
    }
}

/// Runs the Mandelbrot algorithm for 10 iterations on a complex number `c`, then returns the `Growth`.
/// It returns `Stable` if the value hasn't exceeded 2.0 in any direction on the complex plane,
/// and `After(n)`
pub fn explodes_after(c: Complex64) -> Growth {
    for (i, c) in Mandelbrot::new(c).take(ITERATION_BOUND).enumerate() {
        if c.norm() > 2.0 {
            return Growth::After(i);
        }
    }

    Growth::Stable
}

/// Defines a Mandelbrot iterator, which does the iteration of repeatedly applying the function to itself
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mandelbrot {
    c: Complex64, // is constant after creation
    z: Complex64,
}

impl Mandelbrot {
    pub const fn new(c: Complex64) -> Mandelbrot {
        Mandelbrot {
            c,
            z: Complex64::new(0.0, 0.0),
        }
    }

    pub const fn from_re(r: f64) -> Mandelbrot {
        Mandelbrot::new(Complex64::new(r, 0.0))
    }

    pub const fn from_im(i: f64) -> Mandelbrot {
        Mandelbrot::new(Complex64::new(0.0, i))
    }
}

impl Iterator for Mandelbrot {
    type Item = Complex64;

    fn next(&mut self) -> Option<Self::Item> {
        self.z = self.z.powi(2) + self.c;

        Some(self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_iterator() {
        let actual: Complex64 = Mandelbrot::from_re(0.0).nth(10).unwrap();
        let expected: Complex64 = 0.0.into();

        assert_eq!(actual, expected);

        let actual: Complex64 = Mandelbrot::from_re(1.0).nth(3).unwrap();
        let expected: Complex64 = 26.0.into();

        assert_eq!(actual, expected);
    }
}
