use clap::ArgAction;

#[derive(clap::Parser)]
#[command(version, long_about = include_str!("cli_long_about.txt"), disable_help_flag = true)]
pub struct Cli {
    #[arg(long, action = clap::ArgAction::Help, help = "show this message")]
    help: Option<bool>,

    #[arg(long, short, help = "use defaults automatically", action = ArgAction::SetTrue)]
    pub defaults: bool,

    #[arg(long, short, default_value_t = 256)]
    pub width: u32,
    #[arg(long, short, default_value_t = 256)]
    pub height: u32,

    #[arg(long, short, help = "random seed")]
    pub seed: Option<u64>,

    #[arg(long, short)]
    pub output_path: Option<String>,

    #[arg(long, short, help = format!("algorithms: {:?}", crate::algo::ALGORITHMS))]
    pub algo: Option<String>,
    #[arg(long, short)]
    pub task_file: Option<String>
}
