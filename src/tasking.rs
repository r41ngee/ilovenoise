use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize};

use crate::save_image;

#[derive(Debug, Deserialize)]
struct TasksConfig {
    pub task: Vec<TaskConfig>
}

#[derive(Deserialize, Debug)]
pub struct TaskConfig {
    pub mode: String,
    pub width: u32,
    pub height: u32,
    pub output: Option<String>,
    pub seed: Option<u64>,
    pub perlin: Option<PerlinConfig>,
}

#[derive(Debug, Deserialize)]
pub struct PerlinConfig {
    pub octaves: Option<u32>,
    pub persistence: Option<f32>,
    pub lacunarity: Option<f64>,
}

impl TaskConfig {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let size = (self.width, self.height);

        let rand_thr = ChaCha8Rng::seed_from_u64(self.seed.unwrap_or_else(||rand::random()));
        let mut algorithm = crate::create_mode(rand_thr, size, &self)?;
        let mut image = crate::image::Image::new(size);

        algorithm.draw(&mut image);

        save_image(&image, self.output.clone().unwrap_or("output.png".to_string()).as_str() )
    }

    pub fn from_args(args: &crate::cli::Cli) -> Self {
        Self {
            mode: args.algo.clone().unwrap_or_default(),
            width: args.width,
            height: args.height,
            output: args.output_path.clone(),
            seed: args.seed,
            perlin: (args.algo.as_deref() == Some("perlin")).then(|| PerlinConfig {
                octaves: args.octaves,
                persistence: args.persistence,
                lacunarity: args.lacunarity,
            }),
        }
    }
}

pub fn load_tasks(path: &str) -> Result<Vec<TaskConfig>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: TasksConfig = toml::from_str(&content)?;
    Ok(config.task)
}