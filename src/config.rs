use num_complex::Complex64;
use ron::de::from_str;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    /// Scale factor represents how much the coordinate number needs to be scaled by
    /// to fit the constraints of the complex number.
    /// This value should be changed to generate images of other sizes.
    ///
    /// **NOTE:** A scale factor of 2000 only works in release mode, use 1200 in debug
    pub scale_factor: i32,
    /// The value for `c`, which is constant in Julia sets.
    /// It is not used when generating the Mandelbrot set.
    pub julia_constant: Complex64,
    /// The iteration depth.
    /// I.e. how many iterations the algorithm will attempt before determining a given point is "stable".
    pub iteration_depth: usize,
}

impl Config {
    pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
        let cfg = read_to_string(path)?;
        let res = from_str::<Config>(&cfg)?;
        Ok(res)
    }

    pub const fn img_height(&self) -> i32 {
        self.scale_factor * 4
    }

    pub const fn img_width(&self) -> i32 {
        self.scale_factor * 4
    }

    pub const fn center_x(&self) -> i32 {
        self.img_width() / 2
    }

    pub const fn center_y(&self) -> i32 {
        self.img_height() / 2
    }

    pub const fn scale_factor_f(&self) -> f64 {
        self.scale_factor as f64
    }
}
