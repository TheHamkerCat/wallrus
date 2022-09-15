use colored::Colorize;
use rand::distributions::{Alphanumeric, DistString};
use std::process::Command;

pub fn program_exists(program: &str) -> bool {
    Command::new("which")
        .arg(&program)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap()
        .success()
}

pub fn ensure_program(program: &str) {
    if !program_exists(program) {
        eprintln!(
            "{} {}",
            program.green(),
            "is not installed, install it and try again!".red()
        );
        std::process::exit(1)
    }
}

pub fn gen_tmpfile() -> String {
    format!(
        "/tmp/{}",
        Alphanumeric.sample_string(&mut rand::thread_rng(), 6)
    )
}

pub fn get_home_dir() -> std::string::String {
    std::env::var("HOME").unwrap()
}
