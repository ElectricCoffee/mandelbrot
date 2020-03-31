#![allow(unused)]
use super::color::*;
use num::Zero;
const ITERATION_BOUND: usize = 48;

pub type Complex = num::Complex<f64>;

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
            After(n) if n >= 40 => VIOLET,
            After(n) if n >= 36 => MAGENTA,
            After(n) if n >= 32 => ROSE,
            After(n) if n >= 28 => RED,
            After(n) if n >= 24 => ORANGE,
            After(n) if n >= 20 => YELLOW,
            After(n) if n >= 16 => CHARTREUSE,
            After(n) if n >= 12 => GREEN,
            After(n) if n >= 08 => SPRING,
            After(n) if n >= 04 => CYAN,
            After(n) if n >= 02 => AZURE,
            _ => BLUE,
        }
    }
}

/// Runs the Mandelbrot algorithm for 10 iterations on a complex number `c`, then returns the `Growth`.
/// It returns `Stable` if the value hasn't exceeded 2.0 in any direction on the complex plane,
/// and `After(n)`
pub fn explodes_after(c: Complex) -> Growth {
    for (i, c) in Mandelbrot::new(c).take(ITERATION_BOUND).enumerate() {
        let Complex { re, im } = c;
        if re.abs() > 2.0 || im.abs() > 2.0 {
            return Growth::After(i);
        }
    }

    Growth::Stable
}

/// Defines a Mandelbrot iterator, which does the iteration of repeatedly applying the function to itself
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mandelbrot {
    c: Complex, // is constant after creation
    z: Complex,
}

impl Mandelbrot {
    pub const fn new(c: Complex) -> Mandelbrot {
        Mandelbrot {
            c,
            z: Complex::new(0.0, 0.0),
        }
    }

    pub const fn from_re(r: f64) -> Mandelbrot {
        Mandelbrot::new(Complex::new(r, 0.0))
    }

    pub const fn from_im(i: f64) -> Mandelbrot {
        Mandelbrot::new(Complex::new(0.0, i))
    }
}

impl Iterator for Mandelbrot {
    type Item = Complex;

    fn next(&mut self) -> Option<Self::Item> {
        self.z = self.z.powi(2) + self.c;

        Some(self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    type Complex = num::Complex<f64>;
    #[test]
    fn test_iterator() {
        let actual: Complex = Mandelbrot::from_re(0f64).nth(10).unwrap();
        let expected: Complex = 0f64.into();

        assert_eq!(actual, expected);

        let actual: Complex = Mandelbrot::from_re(1f64).nth(3).unwrap();
        let expected: Complex = 26f64.into();

        assert_eq!(actual, expected);
    }
}
