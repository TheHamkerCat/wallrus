pub mod argparse;
pub mod functions;
mod get;
mod set;

fn main() {
    let args = argparse::arguments().get_matches();

    if let Some((name, subc)) = args.subcommand() {
        match name {
            "set" => set::set_wallpaper(subc).unwrap(),
            "get" => get::get_wallpaper(subc),
            _ => println!("Lost?? try --help"),
        }
    };
}
