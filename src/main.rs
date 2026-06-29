use clap::Parser;
use ilovenoise_core::{
    image::Image,
    tasking::{self, TaskConfig},
    create_mode,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

mod cli;

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
            run_task(&task)?;
        }
        return Ok(());
    }

    let config = task_config_from_cli(&args);

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

fn task_config_from_cli(args: &cli::Cli) -> TaskConfig {
    TaskConfig {
        mode: args.algo.clone().unwrap_or_default(),
        width: args.width,
        height: args.height,
        output: args.output_path.clone(),
        seed: args.seed,
        perlin: (args.algo.as_deref() == Some("perlin")).then_some(tasking::PerlinConfig {
            octaves: args.octaves,
            persistence: args.persistence,
            lacunarity: args.lacunarity,
        }),
    }
}

fn run_task(task: &TaskConfig) -> Result<(), Box<dyn std::error::Error>> {
    let size = (task.width, task.height);
    if !size.0.is_multiple_of(8) || !size.1.is_multiple_of(8) {
        return Err("Width and height must be multiples of 8".into());
    }
    let rand_thr = ChaCha8Rng::seed_from_u64(task.seed.unwrap_or_else(rand::random));
    let mut algorithm = create_mode(rand_thr, size, task)?;
    let mut image = Image::new(size);
    algorithm.draw(&mut image);
    save_image(
        &image,
        task.output.clone().unwrap_or("output.png".to_string()).as_str(),
    )
}

fn save_image(image: &Image, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

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
