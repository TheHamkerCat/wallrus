use clap::ArgMatches;
use colored::Colorize;
use scraper::{Html, Selector};

use rand::seq::SliceRandom;
use std::process::Command as Shell;

pub fn set_wallpaper(subc: &ArgMatches) {
    let query = subc.value_of("query");
    let path = subc.value_of("path");
    let mode = subc.value_of("mode");
    let noxinerama = subc.is_present("noxinerama");

    if let (Some(_), Some(_)) = (query, path) {
        let err = format!(
            "You can't use both {} and {} at the same time! Either supply a UNIX path or give me a query to search for on the internet!",
            "--query".green(), "--path".green()
        );
        return eprintln!("{}", err.red());
    } else if let (None, None) = (query, path) {
        let err = format!(
            "You need to supply either {} or a {} to search for on the internet!",
            "--query".green(),
            "--path".green()
        );
        return eprintln!("{}", err.red());
    }

    if let Some(query) = query {
        set_wall_using_query(query, mode, noxinerama);
    } else if let Some(path) = path {
        set_wall_using_path(path, mode, noxinerama);
    }
    println!("query: {:?}", query);
    println!("path: {:?}", path);
    println!("mode: {:?}", mode);
}

fn set_wall_using_path(path: &str, mode: Option<&str>, noxinerama: bool) {
    let mut args = Vec::new();

    if noxinerama {
        args.push("--no-xinerama".to_owned());
    } else {
        let mut og_arg = "--bg-".to_owned();
        og_arg.push_str(mode.unwrap());

        args.push(og_arg.clone())
    }

    args.push(path.to_owned());

    Shell::new("feh")
        .args(args)
        .status()
        .expect("Failed to set wallpaper because of feh issue.");
}

fn set_wall_using_query(query: &str, mode: Option<&str>, noxinerama: bool) {
    let url = format!(
        "https://www.wallpaperflare.com/search?wallpaper={}&page=1",
        query
    );
    let resp = reqwest::blocking::get(url)
        .expect("HTTP request failed.")
        .text();

    let html = match resp {
        Ok(text) => text,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let fragmant = Html::parse_document(html.as_str());
    let selector = Selector::parse("a[itemprop=\"url\"]").unwrap();

    let mut pages = Vec::new();

    for element in fragmant.select(&selector) {
        let page_url = element.value().attr("href").unwrap_or("a");

        if page_url.len() >= 10 {
            pages.push(page_url)
        }
    }
    let page_url = pages.choose(&mut rand::thread_rng());

    let mut page_url = (*page_url.unwrap()).to_owned();
    page_url.push_str("/download/");

    let resp = reqwest::blocking::get(page_url)
        .expect("HTTP request failed.")
        .text();

    let html = match resp {
        Ok(text) => text,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let fragmant = Html::parse_document(html.as_str());
    let selector = Selector::parse("img#show_img").unwrap();

    let element = fragmant.select(&selector).next();

    let image_url = match element {
        Some(element) => match element.value().attr("src") {
            Some(element) => element,
            None => panic!("HTTP request failed."),
        },
        None => panic!("HTTP request failed."),
    };

    println!("{}", image_url)
}
