mod mandelbrot;
use std::{fs::File, io::BufWriter, path::Path};

const SCALE_FACTOR: i32 = 2000;
const IMG_HEIGHT: i32 = 4 * SCALE_FACTOR;
const IMG_WIDTH: i32 = 4 * SCALE_FACTOR;
const CENTER_X: i32 = IMG_WIDTH / 2;
const CENTER_Y: i32 = IMG_HEIGHT / 2;
const SCALE_FACTORF: f64 = SCALE_FACTOR as f64;

fn px_to_c(x: i32, y: i32) -> mandelbrot::Complex {
    let re = (x - CENTER_X) as f64 / SCALE_FACTORF;
    let im = (y - CENTER_Y) as f64 / SCALE_FACTORF;

    mandelbrot::Complex { re, im }
}

fn linear_to_coord(i: i32) -> (i32, i32) {
    let x = i % IMG_WIDTH;
    let y = i / IMG_WIDTH;

    (x, y)
}

fn mk_encoder(path: &str) -> png::Encoder<BufWriter<File>> {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMG_WIDTH as u32, IMG_HEIGHT as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
}

fn generate_data() -> Vec<u8> {
    println!("Generating image data, hold tight...");
    let mut data = Vec::new();

    for i in 0..(IMG_HEIGHT * IMG_WIDTH) {
        // convert index in array to xy-plane
        let (x, y) = linear_to_coord(i);
        // convert xy coordinates to complex number with 0+0i in the middle of the image
        let c = px_to_c(x, y);
        // check how many iterations it takes to exceed |2| or |2i| and turn it into some colours
        let rgb = mandelbrot::explodes_after(c).to_rgb();
        // push the [u8; 3] colours to the vector
        data.push(rgb);
    }

    println!("Generating done, verifying integrity.");
    // assertion fails, 192008000 != 192008246, but why?
    assert_eq!(data.len() as i32, IMG_WIDTH * IMG_HEIGHT);
    println!("Integrity verified, flattening.");
    data.iter().flatten().cloned().collect()
}

fn main() {
    let title = format!("mandelbrot_{}x{}.png", IMG_WIDTH, IMG_HEIGHT);
    println!("Generating {}.", title);
    let encoder = mk_encoder(&title);
    let mut writer = encoder.write_header().unwrap();
    let data = generate_data();
    //assert_eq!(data.len() as i32, IMG_WIDTH * IMG_HEIGHT * 3);
    println!("Writing image data...");
    writer.write_image_data(&data).unwrap();
    println!("Done.");
}
