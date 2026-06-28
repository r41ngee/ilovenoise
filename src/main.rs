use clap::Parser;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::algo::Aglorithm;
use crate::image::Image;

mod algo;
mod cli;
mod image;
mod tasking;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::parse();

    if let Some(shell) = args.completions {
        use clap::CommandFactory;
        let mut cmd = cli::Cli::command();
        clap_complete::generate(shell, &mut cmd, env!("CARGO_PKG_NAME"), &mut std::io::stdout());
        return Ok(());
    }

    if let Some(taskfile) = &args.task_file {
        let tasks = tasking::load_tasks(taskfile)?;
        for task in tasks {
            task.run()?;
        }
        return Ok(());
    }

    let config = tasking::TaskConfig::from_args(&args);

    let size = (config.width, config.height);
    if !size.0.is_multiple_of(8) || !size.1.is_multiple_of(8) {
        return Err("Width and height must be multiples of 8".into());
    }
    let rand_thr = ChaCha8Rng::seed_from_u64(config.seed.unwrap_or_else(rand::random));

    let mut image = Image::new(size);
    let mut algorithm = create_mode(rand_thr, size, &config)?;
    algorithm.draw(&mut image);

    let default_path = "output.png".to_string();
    save_image(&image, &config.output.unwrap_or(default_path))?;

    Ok(())
}

fn create_mode(
    rand_thr: ChaCha8Rng,
    size: (u32, u32),
    config: &tasking::TaskConfig,
) -> Result<Box<dyn Aglorithm>, Box<dyn std::error::Error>> {
    match config.mode.to_lowercase().as_str() {
        "random" => Ok(Box::new(algo::random_noise::RandomNoise::new(rand_thr))),
        "perlin" => {
            let p = &config.perlin;
            Ok(Box::new(algo::perlin::Perlin::new(
                size,
                rand_thr,
                p.as_ref().and_then(|c| c.octaves),
                p.as_ref().and_then(|c| c.persistence),
                p.as_ref().and_then(|c| c.lacunarity),
            )))
        }
        _ => Err(format!("unknown algorithm: {}. Use some of: {:?}", config.mode, algo::ALGORITHMS).into()),
    }
}

pub(crate)
fn save_image(image: &Image, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&filepath);
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image.size.0, image.size.1);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    let image_data = image.as_bytes();

    writer.write_image_data(&image_data)?;
    writer.finish()?;
    Ok(())
}