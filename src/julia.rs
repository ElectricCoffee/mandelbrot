#![allow(unused)]
use super::color::{BLACK, COLORS};

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

/// Defines a Julia iterator, which does the iteration of repeatedly applying the function to itself.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Julia {
    c: Complex64, // is constant after creation
    z: Complex64,
    power: i32,
}

impl Julia {
    /// Creates a new Julia Set Iterator.
    /// `c` is the constant which is added onto the complex number `z`.
    /// `z` is the initial input to the function fc(z) = z^2 + c, from which Mandelbrot and Julia are typically derived.
    pub const fn new(c: Complex64, z: Complex64) -> Julia {
        Julia { c, z, power: 2 }
    }

    /// Creates a new Julia Set Iterator with the initial `z` value set to 0
    pub const fn new_mandelbrot(c: Complex64) -> Julia {
        Julia::new(c, Complex64::new(0.0, 0.0))
    }

    /// Sets the power to a different value than 2 for when you want to generate images that aren't the standard set.
    /// The value is an integer for performance reasons, a floating point power is much slower.
    pub const fn set_pow(self, power: i32) -> Julia {
        Julia { power, ..self }
    }

    /// Runs the iterator for at most `iteration_depth` iterations, after which the point is declared "stable".
    pub fn get_growth(&self, iteration_depth: usize) -> Growth {
        for (i, c) in self.take(iteration_depth).enumerate() {
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
