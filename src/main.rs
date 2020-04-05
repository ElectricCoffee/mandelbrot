mod color;
/// Holds the config struct, which is used to store the deserialisation of config.ron
mod config;
/// Holds the actual logic for calculating the points of the Julia/Mandelbrot sets
mod julia;
use config::{Config, Mode};
use julia::Julia;
use num_complex::Complex64;
use png::{EncodingError, Writer};
use std::io::{BufWriter, Write};
use std::{fs::File, path::Path};

/// Converts a coordinate to a complex number centred in the middle of the image.
fn px_to_c((x, y): (i32, i32), cfg: &Config) -> Complex64 {
    let re = (x - cfg.center_x()) as f64 / cfg.scale_factor_f();
    let im = (y - cfg.center_y()) as f64 / cfg.scale_factor_f();

    Complex64 { re, im }
}

/// Converts a linear index to a coordinate in the x/y plane.
fn linear_to_coord(i: i32, cfg: &Config) -> (i32, i32) {
    let x = i % cfg.img_width();
    let y = i / cfg.img_height();

    (x, y)
}

/// Creates an encoder with the relevant parameters
fn mk_writer(path: &str, cfg: &Config) -> Result<Writer<impl Write>, EncodingError> {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, cfg.img_width() as u32, cfg.img_height() as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header()
}

/// Runs the mandelbrot algorithm and generates the vector of 8-bit data
/// that constitutes the colour information in the image
fn generate_data(cfg: &Config) -> Vec<u8> {
    println!("Generating image data, hold tight...");
    let colors = (cfg.stable_color, cfg.coloring.clone().build());
    (0..(cfg.img_height() * cfg.img_width()))
        .map(|i| {
            let coord = linear_to_coord(i, cfg);
            let z = px_to_c(coord, cfg);
            let iterator = match cfg.mode {
                Mode::Julia(re, im) => Julia::new(Complex64::new(re, im), z).set_pow(cfg.power),
                Mode::Mandelbrot => Julia::new_mandelbrot(z).set_pow(cfg.power),
            };

            iterator
                .get_growth(cfg.iteration_depth)
                .to_rgb(&colors)
                .to_vec()
        })
        .flatten()
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading config file...");
    let config = Config::load("config.ron")?;
    let title = format!(
        "{}_{}x{}.png",
        config.mode,
        config.img_width(),
        config.img_height()
    );
    println!("Generating {}.", title);
    let mut writer = mk_writer(&title, &config).unwrap();
    let data = generate_data(&config);
    println!("Writing image data...");

    // internal assertion fails, 192008000 != 192008246, but why?
    writer.write_image_data(&data).unwrap();
    println!("Done.");

    Ok(())
}
