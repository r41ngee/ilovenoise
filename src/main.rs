use clap::Parser;
use rand::{SeedableRng};
use rand_chacha::ChaCha8Rng;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::algo::Aglorithm;

mod cli;
mod util;
mod image;
mod algo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argparser = cli::Cli::parse();
    let (width, height) = (argparser.width, argparser.height);

    let mut image = image::Image::new(width, height);
    let rand_thr = ChaCha8Rng::seed_from_u64(
        argparser.seed.unwrap_or_else( rand::random )
    );

    let mut algorithm: Box<dyn Aglorithm> = Box::new(algo::random_noise::RandomNoise::new(rand_thr));
    algorithm.draw(&mut image);

    let savepath_name = argparser.output_path.unwrap_or("output.png".to_string());
    let path = Path::new(&savepath_name);
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    let image_data = image.as_bytes();

    writer.write_image_data(&image_data)?;
    writer.finish()?;

    println!("Succeed!");
    Ok(())
}
