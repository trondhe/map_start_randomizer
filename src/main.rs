#![allow(dead_code, unused_variables)]

use std::{fs::File, io::Write};

use clap::Parser;
use map_json::{MapJson, MapPoint, MapPointValue};
use rand::Rng;

mod map_json;

const MAX_LOOP_COUNT: usize = 1000;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser)]
    name: Option<String>,
}

#[derive(PartialEq, Debug)]
enum RangeType {
    Alpha,
    Numeric,
}

#[derive(PartialEq, Debug)]
struct Map {
    name: String,
    x_type: RangeType,
    y_type: RangeType,
    x_range: (isize, isize),
    y_range: (isize, isize),
    limitations: Vec<(isize, isize)>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            name: "generic".to_owned(),
            x_type: RangeType::Alpha,
            y_type: RangeType::Alpha,
            x_range: (0, 7),
            y_range: (8, 15),
            limitations: vec![],
        }
    }
}

impl Map {
    fn generate_coords(&self) -> (isize, isize) {
        let (x_min, x_max) = (self.x_range.0, self.x_range.1);
        let (y_min, y_max) = (self.y_range.0, self.y_range.1);

        assert!(x_min <= x_max);
        assert!(y_min <= y_max);

        let mut loop_count = 0;
        loop {
            let x = rand::thread_rng().gen_range(x_min..x_max);
            let y = rand::thread_rng().gen_range(y_min..y_max);
            if !self.limitations.contains(&(x, y)) {
                return (x, y);
            }
            loop_count += 1;
            assert!(loop_count > MAX_LOOP_COUNT);
        }
    }

    fn print_coords(&self) {
        println!("{}", self.name);
        let coords = self.generate_coords();
        if self.x_type == RangeType::Numeric {
            println!("x: {}", coords.0);
        } else {
            let alpha = (coords.0 as u8 + 65) as char;
            println!("x: {}", alpha);
        }
        if self.y_type == RangeType::Numeric {
            println!("y: {}", coords.1);
        } else {
            let alpha = (coords.1 as u8 + 65) as char;
            println!("y: {}", alpha);
        }
    }
}

fn write_map_json() {
    let mut vec = Vec::new();
    for i in 0..3 {
        let mut map = MapJson::default();
        map.limitations.push(MapPoint {
            point: (MapPointValue::Alpha('A'), MapPointValue::Alpha('I')),
        });
        map.limitations.push(MapPoint {
            point: (MapPointValue::Alpha('B'), MapPointValue::Alpha('J')),
        });
        vec.push(map);
    }
    let result = serde_json::to_string(&vec);
    let result = result.unwrap();
    let mut f = File::create("output.json").expect("Unable to create file");
    f.write_all(result.as_bytes())
        .expect("could not write data to file");
}

fn main() {
    let args = Args::parse();

    let map: Map = {
        if let Some(name) = args.name {
            map_json::special_map(name).unwrap_or_default()
        } else {
            Map::default()
        }
    };

    map.print_coords();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_deserialize_works() {
        let map = map_json::MapJson::default();
        let map_string = serde_json::to_string(&map).unwrap();
        let deserialized_map: map_json::MapJson = serde_json::from_str(&map_string).unwrap();
        assert_eq!(map, deserialized_map);
    }

    #[test]
    fn defaults_are_equal() {
        let map_json = Map::from(map_json::MapJson::default());
        let map = Map::default();
        assert_eq!(map, map_json);
    }
}
