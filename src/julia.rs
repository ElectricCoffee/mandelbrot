#![allow(unused)]
use super::color::{BLACK, COLORS};
const ITERATION_BOUND: usize = 120;

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

pub fn julia_growth(c: Complex64, z: Complex64) -> Growth {
    for (i, c) in Julia::new(c, z).take(ITERATION_BOUND).enumerate() {
        if c.norm() > 2.0 {
            return Growth::After(i);
        }
    }

    Growth::Stable
}

/// Runs the Mandelbrot algorithm for 10 iterations on a complex number `c`, then returns the `Growth`.
/// It returns `Stable` if the value hasn't exceeded 2.0 in any direction on the complex plane,
/// and `After(n)`
pub fn mandelbrot_growth(c: Complex64) -> Growth {
    for (i, c) in Julia::mandelbrot(c).take(ITERATION_BOUND).enumerate() {
        if c.norm() > 2.0 {
            return Growth::After(i);
        }
    }

    Growth::Stable
}

/// Defines a Mandelbrot iterator, which does the iteration of repeatedly applying the function to itself
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Julia {
    c: Complex64, // is constant after creation
    z: Complex64,
}

impl Julia {
    pub const fn new(c: Complex64, z: Complex64) -> Julia {
        Julia { c, z }
    }

    pub const fn mandelbrot(c: Complex64) -> Julia {
        Julia {
            c,
            z: Complex64::new(0.0, 0.0),
        }
    }
}

impl Iterator for Julia {
    type Item = Complex64;

    fn next(&mut self) -> Option<Self::Item> {
        self.z = self.z.powi(2) + self.c;

        Some(self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn from_real(n: f64) -> Complex64 {
        Complex64::new(n, 0.0)
    }

    #[test]
    fn test_iterator() {
        let actual: Complex64 = Julia::mandelbrot(from_real(0.0)).nth(10).unwrap();
        let expected: Complex64 = 0.0.into();

        assert_eq!(actual, expected);

        let actual: Complex64 = Julia::mandelbrot(from_real(1.0)).nth(3).unwrap();
        let expected: Complex64 = 26.0.into();

        assert_eq!(actual, expected);
    }
}
