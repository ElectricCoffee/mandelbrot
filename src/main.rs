mod color;
mod julia;
use julia::Julia;
use num_complex::Complex64;
use std::io::{BufWriter, Write};
use std::{fs::File, path::Path};

use png::{EncodingError, Writer};

/// Scale factor represents how much the coordinate number needs to be scaled by
/// to fit the constraints of the complex number.
/// This value should be changed to generate images of other sizes.
///
/// **NOTE:** A scale factor of 2000 only works in release mode, use 1200 in debug
const SCALE_FACTOR: i32 = 2000;
// const SCALE_FACTOR: i32 = 1200; // works in debug
const IMG_HEIGHT: i32 = 4 * SCALE_FACTOR;
const IMG_WIDTH: i32 = 4 * SCALE_FACTOR;
const CENTER_X: i32 = IMG_WIDTH / 2;
const CENTER_Y: i32 = IMG_HEIGHT / 2;

///Scale Factor F is merely a convenience so I don't have to carry "as f64" around everywhere
const SCALE_FACTORF: f64 = SCALE_FACTOR as f64;

const JULIA_CONSTANT: Complex64 = Complex64::new(-0.8, 0.156);
//const JULIA_CONSTANT: Complex64 = Complex64::new(-0.4, 0.6);
//const JULIA_CONSTANT: Complex64 = Complex64::new(0.285, 0.01);
//const JULIA_CONSTANT: Complex64 = Complex64::new(-0.835, -0.2321);
//const JULIA_CONSTANT: Complex64 = Complex64::new(1.0, 0.0);

/// Converts a coordinate to a complex number centred in the middle of the image.
fn px_to_c(x: i32, y: i32) -> Complex64 {
    let re = (x - CENTER_X) as f64 / SCALE_FACTORF;
    let im = (y - CENTER_Y) as f64 / SCALE_FACTORF;

    Complex64 { re, im }
}

/// Converts a linear index to a coordinate in the x/y plane.
fn linear_to_coord(i: i32) -> (i32, i32) {
    let x = i % IMG_WIDTH;
    let y = i / IMG_WIDTH;

    (x, y)
}

/// Creates an encoder with the relevant parameters
fn mk_writer(path: &str) -> Result<Writer<impl Write>, EncodingError> {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMG_WIDTH as u32, IMG_HEIGHT as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header()
}

/// Runs the mandelbrot algorithm and generates the vector of 8-bit data
/// that constitutes the colour information in the image
fn generate_data(constant: Complex64) -> Vec<u8> {
    println!("Generating image data, hold tight...");
    (0..(IMG_HEIGHT * IMG_WIDTH))
        .map(|i| {
            let (x, y) = linear_to_coord(i);
            let z = px_to_c(x, y);
            Julia::new(constant, z).get_growth().to_rgb().to_vec()
            //Julia::new_mandelbrot(z).get_growth().to_rgb().to_vec() // to generate mandelbrot
        })
        .flatten()
        .collect()
}

fn main() {
    let title = format!("julia_{}_{}x{}.png", JULIA_CONSTANT, IMG_WIDTH, IMG_HEIGHT);
    println!("Generating {}.", title);
    let mut writer = mk_writer(&title).unwrap();
    let data = generate_data(JULIA_CONSTANT);
    println!("Writing image data...");

    // internal assertion fails, 192008000 != 192008246, but why?
    writer.write_image_data(&data).unwrap();
    println!("Done.");
}
