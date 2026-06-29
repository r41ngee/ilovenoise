use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize};

use crate::save_image;

#[derive(Debug, Deserialize)]
struct TasksConfig {
    pub task: Vec<TaskConfig>
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskConfig {
    pub mode: String,
    pub width: u32,
    pub height: u32,
    pub output: Option<String>,
    pub seed: Option<u64>,
    pub perlin: Option<PerlinConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PerlinConfig {
    pub octaves: Option<u32>,
    pub persistence: Option<f32>,
    pub lacunarity: Option<f64>,
}

impl TaskConfig {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let size = (self.width, self.height);

        let rand_thr = ChaCha8Rng::seed_from_u64(self.seed.unwrap_or_else(rand::random));
        let mut algorithm = crate::create_mode(rand_thr, size, self)?;
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
            perlin: (args.algo.as_deref() == Some("perlin")).then_some(PerlinConfig {
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

#[cfg(test)]
mod tests {
    use clap::Parser;
    use super::*;

    #[test]
    fn parse_full_perlin_task() {
        let toml_str = r#"
[[task]]
mode = "Perlin"
width = 512
height = 512
seed = 42
output = "perlin.png"

[task.perlin]
octaves = 6
persistence = 0.3
lacunarity = 2.5
"#;
        let config: TasksConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.task.len(), 1);
        let task = &config.task[0];
        assert_eq!(task.mode, "Perlin");
        assert_eq!(task.width, 512);
        assert_eq!(task.height, 512);
        assert_eq!(task.seed, Some(42));
        assert_eq!(task.output.as_deref(), Some("perlin.png"));
        let perlin = task.perlin.as_ref().unwrap();
        assert_eq!(perlin.octaves, Some(6));
        assert!((perlin.persistence.unwrap() - 0.3).abs() < f32::EPSILON);
        assert!((perlin.lacunarity.unwrap() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn parse_minimal_random_task() {
        let toml_str = r#"
[[task]]
mode = "Random"
width = 256
height = 256
"#;
        let config: TasksConfig = toml::from_str(toml_str).unwrap();
        let task = &config.task[0];
        assert_eq!(task.mode, "Random");
        assert_eq!(task.width, 256);
        assert_eq!(task.height, 256);
        assert!(task.seed.is_none());
        assert!(task.output.is_none());
        assert!(task.perlin.is_none());
    }

    #[test]
    fn parse_perlin_without_perlin_config() {
        let toml_str = r#"
[[task]]
mode = "Perlin"
width = 128
height = 128
"#;
        let config: TasksConfig = toml::from_str(toml_str).unwrap();
        let task = &config.task[0];
        assert!(task.perlin.is_none());
    }

    #[test]
    fn parse_perlin_partial_config() {
        let toml_str = r#"
[[task]]
mode = "Perlin"
width = 256
height = 256
seed = 123
output = "test.png"

[task.perlin]
octaves = 3
"#;
        let config: TasksConfig = toml::from_str(toml_str).unwrap();
        let task = &config.task[0];
        assert_eq!(task.seed, Some(123));
        assert_eq!(task.output.as_deref(), Some("test.png"));
        let perlin = task.perlin.as_ref().unwrap();
        assert_eq!(perlin.octaves, Some(3));
        assert!(perlin.persistence.is_none());
        assert!(perlin.lacunarity.is_none());
    }

    #[test]
    fn parse_multiple_tasks() {
        let toml_str = r#"
[[task]]
mode = "Random"
width = 64
height = 64

[[task]]
mode = "Perlin"
width = 128
height = 128
"#;
        let config: TasksConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.task.len(), 2);
        assert_eq!(config.task[0].mode, "Random");
        assert_eq!(config.task[1].mode, "Perlin");
    }

    #[test]
    fn parse_missing_width_is_error() {
        let toml_str = r#"
[[task]]
mode = "Random"
height = 256
"#;
        let result: Result<TasksConfig, _> = toml::from_str(toml_str);
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_mode_is_error() {
        let toml_str = r#"
[[task]]
width = 256
height = 256
"#;
        let result: Result<TasksConfig, _> = toml::from_str(toml_str);
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_height_is_error() {
        let toml_str = r#"
[[task]]
mode = "Random"
width = 256
"#;
        let result: Result<TasksConfig, _> = toml::from_str(toml_str);
        assert!(result.is_err());
    }

    #[test]
    fn from_args_perlin_full() {
        let args = crate::cli::Cli::try_parse_from([
            "ilovenoise",
            "--algo", "perlin",
            "--width", "100",
            "--height", "200",
            "--seed", "42",
            "--output-path", "out.png",
            "--octaves", "6",
            "--persistence", "0.5",
            "--lacunarity", "2.0",
        ]).unwrap();
        let config = TaskConfig::from_args(&args);
        assert_eq!(config.mode, "perlin");
        assert_eq!(config.width, 100);
        assert_eq!(config.height, 200);
        assert_eq!(config.seed, Some(42));
        assert_eq!(config.output.as_deref(), Some("out.png"));
        let p = config.perlin.unwrap();
        assert_eq!(p.octaves, Some(6));
        assert!((p.persistence.unwrap() - 0.5).abs() < f32::EPSILON);
        assert!((p.lacunarity.unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn from_args_random_has_no_perlin() {
        let args = crate::cli::Cli::try_parse_from([
            "ilovenoise",
            "--algo", "random",
        ]).unwrap();
        let config = TaskConfig::from_args(&args);
        assert_eq!(config.mode, "random");
        assert!(config.perlin.is_none());
    }

    #[test]
    fn from_args_perlin_defaults() {
        let args = crate::cli::Cli::try_parse_from([
            "ilovenoise",
            "--algo", "perlin",
        ]).unwrap();
        let config = TaskConfig::from_args(&args);
        let p = config.perlin.unwrap();
        assert!(p.octaves.is_none());
        assert!(p.persistence.is_none());
        assert!(p.lacunarity.is_none());
    }
}