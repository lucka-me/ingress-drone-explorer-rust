use clap::Parser;

use super::explorer;
use super::geography;

pub fn execcute() {
    let arguments = Arguments::parse();

    let mut explorer = explorer::Explorer::new();
    explorer.load_portals(&arguments.portal_list_paths);
    if let Some(path) = arguments.key_list_path {
        explorer.load_keys(&path)
    }
    explorer.explore_from(arguments.start);
    explorer.report();
    if let Some(path) = arguments.output_drawn_items_path {
        explorer.save_drawn_items_to(&path)
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Arguments {
    #[arg(
        num_args(1..),
        value_name = "portal-list-file",
        value_hint = clap::ValueHint::AnyPath,
        help = "Paths of portal list files"
    )]
    portal_list_paths: Vec<std::path::PathBuf>,

    #[arg(
        long, short,
        value_name = "longitude,latitude",
        value_parser = parse_lla,
        help = "The starting point"
    )]
    start: geography::lla::Coordinate,

    #[arg(
        long, short,
        value_name = "key-list",
        value_hint = clap::ValueHint::FilePath,
        help = "Path of key list file"
    )]
    key_list_path: Option<std::path::PathBuf>,

    #[arg(
        long = "output-drawn-items",
        value_name = "path",
        value_hint = clap::ValueHint::FilePath,
        help = "Path of drawn items file to output"
    )]
    output_drawn_items_path: Option<std::path::PathBuf>,
}

fn parse_lla(arg: &str) -> Result<geography::lla::Coordinate, std::num::ParseFloatError> {
    let components: Vec<&str> = arg.split(',').collect();
    let lng: f64 = components[0].parse()?;
    let lat: f64 = components[1].parse()?;
    return Ok(geography::lla::Coordinate { lng, lat });
}