use clap_complete::Shell;

#[derive(clap::Parser)]
#[command(version, long_about = include_str!("cli_long_about.txt"), disable_help_flag = true)]
pub struct Cli {
    #[arg(long, action = clap::ArgAction::Help, help = "show this message")]
    help: Option<bool>,

    #[arg(long, short = 'w', default_value_t = 256, help = "Image width (must be multiple of 8)")]
    pub width: u32,
    #[arg(long, short = 'h', default_value_t = 256, help = "Image height (must be multiple of 8)")]
    pub height: u32,

    #[arg(long, short, help = "Random seed for reproducibility")]
    pub seed: Option<u64>,

    #[arg(long, short, help = "Output PNG path")]
    pub output_path: Option<String>,

    #[arg(
        long,
        short,
        help = "Noise algorithm: random | perlin",
        required_unless_present = "task_file"
    )]
    pub algo: Option<String>,
    #[arg(long, short, help = "TOML file with batch tasks")]
    pub task_file: Option<String>,

    #[arg(long, help = "Generate shell completions (bash|zsh|fish|powershell|elvish)")]
    pub completions: Option<Shell>,

    #[arg(long, help = "Number of fBM octaves (Perlin only)")]
    pub octaves: Option<u32>,
    #[arg(long, help = "Amplitude multiplier per octave (Perlin only)")]
    pub persistence: Option<f32>,
    #[arg(long, help = "Frequency multiplier per octave (Perlin only)")]
    pub lacunarity: Option<f64>,
}
