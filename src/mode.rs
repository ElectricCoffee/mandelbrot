use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The rendering mode for the program.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
    /// `Julia(re, im)` specifies the constant `c` which is used when generating a Julia set at that position
    Julia(f64, f64),
    /// `Mandelbrot` specifies the set to be generated as a Mandelbrot set.
    /// There is no associated data here, because where Julia sets treat `c` as a constant,
    /// Mandelbrot sets treat `c` as the variable, thus `c` is found at runtime.
    Mandelbrot,
}

impl Display for Mode {
    /// Helper formatter to pretty-print filenames
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Julia(re, im) => write!(f, "julia_{}{:+}i", re, im),
            Mode::Mandelbrot => write!(f, "mandelbrot"),
        }
    }
}
