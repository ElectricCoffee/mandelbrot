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

/// Defines a Mandelbrot iterator, which does the iteration of repeatedly applying the function to itself
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Julia {
    c: Complex64, // is constant after creation
    z: Complex64,
    power: i32,
}

impl Julia {
    pub const fn new(c: Complex64, z: Complex64) -> Julia {
        Julia { c, z, power: 2 }
    }

    pub const fn new_mandelbrot(c: Complex64) -> Julia {
        Julia {
            c,
            z: Complex64::new(0.0, 0.0),
            power: 2,
        }
    }

    pub const fn set_pow(self, power: i32) -> Julia {
        Julia { power, ..self }
    }

    pub fn get_growth(&self) -> Growth {
        for (i, c) in self.take(ITERATION_BOUND).enumerate() {
            if c.norm() >= 2.0 {
                return Growth::After(i);
            }
        }

        Growth::Stable
    }
}

impl Iterator for Julia {
    type Item = Complex64;

    fn next(&mut self) -> Option<Self::Item> {
        self.z = self.z.powi(self.power) + self.c;

        Some(self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        let actual: Complex64 = Julia::new_mandelbrot(0.0.into()).nth(10).unwrap();
        let expected: Complex64 = 0.0.into();

        assert_eq!(actual, expected);

        let actual: Complex64 = Julia::new_mandelbrot(1.0.into()).nth(3).unwrap();
        let expected: Complex64 = 26.0.into();

        assert_eq!(actual, expected);
    }
}
