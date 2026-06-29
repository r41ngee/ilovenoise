use clap::Parser;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use ilovenoise::*;

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

    let mut image = image::Image::new(size);
    let mut algorithm = create_mode(rand_thr, size, &config)?;
    algorithm.draw(&mut image);

    let default_path = "output.png".to_string();
    save_image(&image, &config.output.unwrap_or(default_path))?;

    Ok(())
}
