use clap::ArgMatches;
use colored::Colorize;
use rand::distributions::{Alphanumeric, DistString};
use rand::seq::SliceRandom;
use scraper::{Html, Selector};

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
        let output = set_wall_using_query(query, mode, noxinerama);

        match output {
            Ok(()) => (),
            Err(e) => panic!("{}", e),
        }
    } else if let Some(path) = path {
        set_wall_using_path(path, mode, noxinerama);
    }
}

fn set_wall_using_path(
    path: &str,
    mode: Option<&str>,
    noxinerama: bool,
) {
    let mut args = Vec::new();

    args.push(format!("--bg-{}", mode.unwrap()));
    args.push(path.to_owned());

    if noxinerama {
        args.push("--no-xinerama".to_owned());
    }

    Shell::new("feh")
        .args(args)
        .status()
        .expect("Failed to set wallpaper because of feh issue.");
}

#[tokio::main]
pub async fn set_wall_using_query(
    query: &str,
    mode: Option<&str>,
    noxinerama: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Fetching wallpaper...".yellow());

    let url = if query.to_lowercase() == "trending" {
        "https://www.wallpaperflare.com/".to_owned()
    } else {
        format!(
            "https://www.wallpaperflare.com/search?wallpaper={}&page=1",
            query
        )
    };
    // get html and parse it
    let html = reqwest::get(url).await?.text().await?;
    let fragmant = Html::parse_document(html.as_str());
    let selector = Selector::parse("a[itemprop=\"url\"]").unwrap();

    // each page is a webpage to a wallpaper
    let mut pages = Vec::new();

    for element in fragmant.select(&selector) {
        let page_url = element.value().attr("href").unwrap_or("a");

        if page_url.len() >= 10 {
            pages.push(page_url)
        }
    }

    // get a random page
    let page_url = pages.choose(&mut rand::thread_rng());

    let mut page_url =
        (*page_url.ok_or("Couln't Parse HTML")?).to_owned();

    page_url.push_str("/download/");

    // get direct url of wallpaper from webpage
    let html = reqwest::get(page_url).await?.text().await?;

    let fragmant = Html::parse_document(html.as_str());
    let element = fragmant
        .select(&Selector::parse("img#show_img").unwrap())
        .next();

    let image_url = element
        .and_then(|i| i.value().attr("src"))
        .ok_or("Couldn't Parse HTML")?;

    // download the image
    let filename = format!(
        "/tmp/{}",
        Alphanumeric.sample_string(&mut rand::thread_rng(), 6)
    );

    let filename = filename.as_str();
    let resp = reqwest::get(image_url).await?;

    let mut file = std::fs::File::create(filename)?;
    let mut content = std::io::Cursor::new(resp.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    println!("{}", "Fetched wallpaper!".blue());
    // set the wallpaper

    set_wall_using_path(filename, mode, noxinerama);
    println!("{}", "Wallpaper set successfully!".green());
    std::fs::remove_file(filename)?;

    Ok(())
    // download the wallpaper and send to set_wall_using_path function, save it too maybe
}
