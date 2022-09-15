use clap::ArgMatches;
use colored::Colorize;
use std::path::Path;

use crate::functions::{ensure_program, get_home_dir};

pub fn get_wallpaper(subc: &ArgMatches) {
    let path = subc.value_of("file").unwrap();
    let fehconf_path = Path::new(&get_home_dir()).join(".fehbg");

    ensure_program("feh");

    if !fehconf_path.exists() {
        return eprintln!(
            "{}",
            "Couldn't find feh config file (.fehbg) in home directory".red()
        );
    }

    let fehconf_content =
        std::fs::read_to_string(fehconf_path).unwrap();

    let wallpaper_file = fehconf_content
        .split('\n')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth_back(0)
        .unwrap()
        .replace('\'', "");

    if !Path::new(&wallpaper_file).exists() {
        return eprintln!(
            "{}",
            "Your current wallpaper file seems missing.".red()
        );
    }

    match std::fs::copy(wallpaper_file, path) {
        Ok(_) => println!("{}", "Wallpaper saved to file!".green()),
        Err(e) => eprintln!("{}", e),
    };
}
