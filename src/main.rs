#![allow(dead_code)]

use clap::Parser;
use map::common::Map;

mod map;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();

    let map: Map = {
        if let Some(name) = args.name {
            map::map_json::special_map(name).unwrap_or_default()
        } else {
            Map::default()
        }
    };

    map::GridPrinter::print(&map);
}
