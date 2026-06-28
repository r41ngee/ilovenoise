// build.rs
use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::fs::create_dir_all;
use std::env;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    // Указываем папку, куда будут сохраняться файлы дополнений.
    // Обычно во время сборки это папка "target/OUT_DIR".
    let out_dir = match env::var("COMPLETIONS_DIR") {
        Ok(dir) => dir,
        Err(_) => return, 
    };

    create_dir_all(&out_dir).unwrap();
    let mut cmd = cli::Cli::command();

    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell] {
        generate_to(shell, &mut cmd, "ilovenoise", &out_dir).unwrap();
    }
}