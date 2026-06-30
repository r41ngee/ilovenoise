pub mod algo;
pub mod image;
pub mod util;

#[cfg(feature = "tasking")]
pub mod tasking;

pub use algo::Algorithm;

#[cfg(feature = "tasking")]
use rand_chacha::ChaCha8Rng;

#[cfg(feature = "tasking")]
pub fn create_mode(
    rand_thr: ChaCha8Rng,
    size: (u32, u32),
    config: &tasking::TaskConfig,
) -> Result<Box<dyn Algorithm>, Box<dyn std::error::Error>> {
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
