use clap::ArgMatches;
use colored::Colorize;

use crate::functions::{get_home_dir, program_exists};

pub fn get_wallpaper(subc: &ArgMatches) {
    let path = subc.value_of("file").unwrap();

    if !program_exists("feh") {
        eprintln!(
            "{}",
            "feh is not installed, install it and try again!".red()
        );
    }

    let fehconf =
        match std::fs::read_to_string(get_home_dir() + "/.fehbg") {
            Ok(fehconf) => fehconf,
            Err(_) => {
                eprintln!(
                    "{}",
                    "Couln't find feh config file.".red()
                );
                return;
            }
        };
    let wallpaper_file = fehconf
        .split('\n')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth_back(0)
        .unwrap()
        .replace('\'', "");

    if !std::path::Path::new(&wallpaper_file).exists() {
        println!(
            "{}",
            "Your current wallpaper file seems missing.".red()
        );
        return;
    }

    match std::fs::copy(wallpaper_file, path) {
        Ok(_) => println!("{}", "Wallpaper saved to file!".green()),
        Err(e) => eprintln!("{}", e),
    };
}
