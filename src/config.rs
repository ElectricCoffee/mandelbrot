use ron::de::from_str;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::read_to_string};

use crate::color::*;
use crate::mode::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Scale factor represents how much the coordinate number needs to be scaled by
    /// to fit the constraints of the complex number.
    /// This value should be changed to generate images of other sizes.
    ///
    /// **NOTE:** A scale factor of 2000 only works in release mode, use 1200 in debug
    pub scale_factor: i32,
    /// The integer power to which `z` is raised.
    /// By default this is set to 2 for the classic fc(z) = z² + c.
    /// Currently only integer powers are supported due to performance.
    #[serde(default = "default_power")]
    pub power: i32,
    /// The Mode has two possible values `Mandelbrot` and `Julia(re, im)`.
    /// The Mandelbrot mode will draw the image as the Mandelbrot set,
    /// while the Julia mode will draw a Julia set with `c` set to _re + im i_.
    pub mode: Mode,
    /// The iteration depth.
    /// I.e. how many iterations the algorithm will attempt before determining a given point is "stable".
    pub iteration_depth: usize,

    /// The colour of stability
    pub stable_color: Color,

    /// The palette of the drawing to be drawn
    pub coloring: Coloring,
}

const fn default_power() -> i32 {
    2
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
