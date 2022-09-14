use clap::ArgMatches;
use colored::Colorize;
use rand::seq::SliceRandom;
use scraper::{Html, Selector};

use crate::functions::{gen_tmpfile, program_exists};
use std::io::Write;
use std::process::Command as Shell;

pub fn set_wallpaper(
    subc: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = subc.value_of("query");
    let path = subc.value_of("path");
    let mode = subc.value_of("mode");
    let noxinerama = subc.is_present("noxinerama");
    let save = subc.value_of("save");
    let cron = subc.value_of("cron");

    // Check if feh is installed in system
    if !program_exists("feh") {
        panic!("{}", "feh is not installed in system [https://archlinux.org/packages/extra/x86_64/feh/]".red())
    }

    if let (Some(_), Some(_)) = (query, path) {
        let err = format!(
            "You can't use both {} and {} at the same time! Either supply a UNIX path or give me a query to search for on the internet!",
            "--query".green(), "--path".green()
        );
        eprintln!("{}", err.red());
        return Ok(());
    } else if let (None, None) = (query, path) {
        let err = format!(
            "You need to supply either {} or a {} to search for on the internet!",
            "--query".green(),
            "--path".green()
        );
        eprintln!("{}", err.red());
        return Ok(());
    }

    if let Some(query) = query {
        let output = set_wall_using_query(
            query, mode, noxinerama, save, cron,
        );

        match output {
            Ok(()) => (),
            Err(e) => panic!("{}", e),
        }
    } else if let Some(path) = path {
        set_wall_using_path(path, mode, noxinerama);
    }
    Ok(())
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

    let status = Shell::new("feh").args(args).status();

    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                panic!(
                    "Failed to set wallpaper due to some feh issue."
                )
            }
        }
        Err(e) => panic!("{}", e),
    }
}

#[tokio::main]
pub async fn set_wall_using_query(
    query: &str,
    mode: Option<&str>,
    noxinerama: bool,
    save: Option<&str>,
    cron: Option<&str>,
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
    let filename = match save {
        Some(name) => name.to_owned(),
        None => gen_tmpfile(),
    };

    let resp = reqwest::get(image_url).await?;

    let mut file = std::fs::File::create(&filename)?;
    let mut content = std::io::Cursor::new(resp.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    println!("{}", "Fetched wallpaper!".blue());

    // set the wallpaper
    set_wall_using_path(&filename[..], mode, noxinerama);
    println!("{}", "Wallpaper set successfully!".green());

    // remove the wallpaper if user doesn't use --save
    if save.is_none() {
        std::fs::remove_file(filename)?;
    }

    if let Some(cron_expression) = cron {
        let exp_len = cron_expression.split_whitespace().count();

        // Check if cron is installed

        if !program_exists("crontab") {
            panic!("{}", "crontab is not installed in system [https://archlinux.org/packages/extra/x86_64/feh/]".red())
        }

        // Check if cron expression is valid
        if !vec![5, 6].contains(&exp_len) {
            panic!("{}", "Cron expression is invalid!".red())
        }

        // current command that user executed
        let mut current_command =
            std::env::args().collect::<Vec<String>>();

        // index of --cron argument, because we want to remove it to avoid cron loop
        let index_of_cron_arg = current_command
            .iter()
            .position(|x| x.to_lowercase().contains("cron"))
            .unwrap();
        current_command.remove(index_of_cron_arg);

        // remove current binary from arguments
        let parameters = current_command[1..].to_vec().join(" ");

        // get actual binary location
        let executable = std::env::current_exe()?
            .into_os_string()
            .into_string()
            .unwrap();

        // this command will be sent to cron to store it
        let final_command = format!(
            "{} DISPLAY=':0' {} {}  # GENERATED BY WALLRUS",
            cron.unwrap(),
            executable,
            parameters
        );

        // get current cron jobs
        let current_cron: Vec<u8> = Shell::new("crontab")
            .arg("-l")
            .output()
            .expect("Cron error")
            .stdout;
        let current_cron: &str =
            std::str::from_utf8(&current_cron)?.trim();

        // remove old wallrus cron line (if exists)
        let current_cron_filtered = if !current_cron.is_empty() {
            let mut splitted_cron =
                current_cron.split('\n').collect::<Vec<&str>>();
            let index_of_old_cron = splitted_cron
                .iter()
                .position(|x| x.contains("GENERATED BY WALLRUS"));

            index_of_old_cron.map(|i| splitted_cron.remove(i));
            splitted_cron.join("\n")
        } else {
            "".to_string()
        };

        // create a cron job
        let final_cron = final_command
            + "\n"
            + current_cron_filtered.as_str()
            + "\n ";

        let tmpfile = gen_tmpfile();
        let mut file = std::fs::File::create(&tmpfile)?;
        file.write_all(final_cron.as_bytes())?;

        // calls `$ crontab filename` which replaces current
        // cronfile to the one we modified
        Shell::new("crontab").arg(&tmpfile).status()?;
    }
    Ok(())
}
