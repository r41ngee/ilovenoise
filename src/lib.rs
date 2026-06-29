pub mod algo;
pub mod cli;
pub mod image;
pub mod tasking;
pub mod util;

pub use algo::Aglorithm;

use rand_chacha::ChaCha8Rng;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn create_mode(
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
        _ => Err(format!(
            "unknown algorithm: {}. Use some of: {:?}",
            config.mode,
            algo::ALGORITHMS
        )
        .into()),
    }
}

pub fn save_image(image: &image::Image, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
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
