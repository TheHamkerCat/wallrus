use rand::distributions::{Alphanumeric, DistString};
use std::process::Command;

pub fn program_exists(program: &str) -> bool {
    let status = Command::new("which")
        .arg(&program)
        .stdout(std::process::Stdio::null())
        .status();

    match status {
        Ok(exit_status) => exit_status.success(),
        Err(e) => panic!("{}", e),
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
