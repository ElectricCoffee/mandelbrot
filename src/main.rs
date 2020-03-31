mod color;
mod mandelbrot;
use std::{fs::File, io::BufWriter, path::Path};

/// Scale factor represents how much the coordinate number needs to be scaled by
/// to fit the constraints of the complex number.
/// This value should be changed to generate images of other sizes.
const SCALE_FACTOR: i32 = 2000;
// const SCALE_FACTOR: i32 = 1200; // works
const IMG_HEIGHT: i32 = 4 * SCALE_FACTOR;
const IMG_WIDTH: i32 = 4 * SCALE_FACTOR;
const CENTER_X: i32 = IMG_WIDTH / 2;
const CENTER_Y: i32 = IMG_HEIGHT / 2;

///Scale Factor F is merely a convenience so I don't have to carry "as f64" around everywhere
const SCALE_FACTORF: f64 = SCALE_FACTOR as f64;

/// Converts a coordinate to a complex number centred in the middle of the image.
fn px_to_c(x: i32, y: i32) -> mandelbrot::Complex {
    let re = (x - CENTER_X) as f64 / SCALE_FACTORF;
    let im = (y - CENTER_Y) as f64 / SCALE_FACTORF;

    mandelbrot::Complex { re, im }
}

/// Converts a linear index to a coordinate in the x/y plane.
fn linear_to_coord(i: i32) -> (i32, i32) {
    let x = i % IMG_WIDTH;
    let y = i / IMG_WIDTH;

    (x, y)
}

/// Creates an encoder with the relevant parameters
fn mk_encoder(path: &str) -> png::Encoder<BufWriter<File>> {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMG_WIDTH as u32, IMG_HEIGHT as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
}

/// Runs the mandelbrot algorithm and generates the vector of 8-bit data
/// that constitutes the colour information in the image
fn generate_data() -> Vec<u8> {
    println!("Generating image data, hold tight...");
    let mut data = Vec::new();

    for i in 0..(IMG_HEIGHT * IMG_WIDTH) {
        let (x, y) = linear_to_coord(i);
        let c = px_to_c(x, y);
        let rgb = mandelbrot::explodes_after(c).to_rgb();
        data.push(rgb);
    }

    println!("Flattening...");
    data.iter().flatten().cloned().collect()
}

fn main() {
    let title = format!("mandelbrot_{}x{}.png", IMG_WIDTH, IMG_HEIGHT);
    println!("Generating {}.", title);
    let encoder = mk_encoder(&title);
    let mut writer = encoder.write_header().unwrap();
    let data = generate_data();
    println!("Writing image data...");

    // internal assertion fails, 192008000 != 192008246, but why?
    writer.write_image_data(&data).unwrap();
    println!("Done.");
}
