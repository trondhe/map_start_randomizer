mod cli;
mod map;

use clap::Parser;
use map::common::Map;

fn main() {
    let args = cli::Args::parse();

    let map: Map = {
        if let Some(name) = args.name {
            map::map_json::special_map(name).unwrap_or_default()
        } else {
            Map::default()
        }
    };

    map::GridPrinter::print(&map);
}
