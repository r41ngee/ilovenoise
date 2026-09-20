use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TasksConfig {
    pub task: Vec<TaskConfig>,
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

pub fn load_tasks(path: &str) -> Result<Vec<TaskConfig>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: TasksConfig = toml::from_str(&content)?;
    Ok(config.task)
}
