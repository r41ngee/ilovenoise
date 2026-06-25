use clap::Parser;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::algo::Aglorithm;

mod algo;
mod cli;
mod image;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argparser = cli::Cli::parse();
    let size = (argparser.width, argparser.height);
    if size.0 % 8 != 0 || size.1 % 8 != 0 {
        return Err("Width and height must be multiples of 8".into());
    }

    let mut image = image::Image::new(size);
    let rand_thr = ChaCha8Rng::seed_from_u64(argparser.seed.unwrap_or_else(rand::random));

    let mut algorithm: Box<dyn Aglorithm> = create_mode(rand_thr, size, argparser.defaults)?;

    algorithm.draw(&mut image);

    let savepath_name = argparser.output_path.unwrap_or("output.png".to_string());
    let path = Path::new(&savepath_name);
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, size.0, size.1);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    let image_data = image.as_bytes();

    writer.write_image_data(&image_data)?;
    writer.finish()?;

    Ok(())
}

fn create_mode(
    rand_thr: ChaCha8Rng,
    size: (u32, u32),
    use_defaults: bool,
) -> Result<Box<dyn Aglorithm>, Box<dyn std::error::Error>> {
    let algorithms = &["Random", "Perlin"];
    let selection = dialoguer::Select::new()
        .with_prompt("Choose your algorithm")
        .items(algorithms)
        .interact()?;

    let box_: Box<dyn Aglorithm> = match selection {
        0 => Box::new(algo::random_noise::RandomNoise::new(rand_thr)),
        1 => {
            if !use_defaults {
                Box::new(algo::perlin::Perlin::new(
                    size,
                    rand_thr,
                    util::input_opt("Octaves", "4")?,
                    util::input_opt("Persistence", "0.5")?,
                    util::input_opt("Lacunarity", "2.0")?,
                ))
            } else {
                Box::new(algo::perlin::Perlin::new(size, rand_thr, None, None, None))
            }
        }
        _ => unreachable!(),
    };

    Ok(box_)
}
