use clap::{arg, Command};

pub fn arguments<'t>() -> Command<'t> {
    Command::new("wallrus")
        .about("A simple wallpaper manager for X11")
        .author("TheHamkerCat <TheHamkerCat@gmail.com>")
        .version("1.0.1")
        .subcommand_required(true)
        .subcommand(
            Command::new("set")
                .about("Set wallpaper by giving an image path or a query to search online.")
                .arg(
                    arg!(-q --query "The query to search for. Pass `trending` for trending wallpapers")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    arg!(-p --path "Path to the wallpaper. Use $HOME instead of ~/")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    arg!(-m --mode "The mode to set the wallpaper.")
                        .takes_value(true)
                        .possible_values(&["center", "fill", "scale", "tile"])
                        .default_value("fill"),
                )
                .arg(
                    arg!(-x --noxinerama "Disables Xinerama support, Making the wallpaper span across all monitors.")
                        .takes_value(false)
                        .required(false)
                )
                .arg(
                    arg!(-s --save "Save the newly fetched wallpaper in a new file. (Only works with --query)")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    arg!(-c --cron "Add current wallpaper command to crontab to change wallpaper automatically. (Only works with --query)")
                        .takes_value(true)
                        .required(false)
                )
                ,
        )
}
