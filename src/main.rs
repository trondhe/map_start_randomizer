#![allow(dead_code)]

use clap::Parser;
use rand::Rng;

mod map_json;

const MAX_LOOP_COUNT: usize = 1000;

type Range = (usize, usize);

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser)]
    name: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum RangeType {
    Alpha,
    Numeric,
}

#[derive(PartialEq, Debug)]
struct Map {
    name: String,
    x_type: RangeType,
    y_type: RangeType,
    x_range: Range,
    y_range: Range,
    limitations: Vec<Range>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            name: "generic".to_owned(),
            x_type: RangeType::Alpha,
            y_type: RangeType::Alpha,
            x_range: ('A' as usize, 'H' as usize),
            y_range: ('I' as usize, 'P' as usize),
            limitations: vec![],
        }
    }
}

impl Map {
    fn x_size(&self) -> usize {
        self.x_range.1 - self.x_range.0
    }

    fn y_size(&self) -> usize {
        self.y_range.1 - self.y_range.0
    }

    fn generate_coords(&self) -> Range {
        let mut loop_count = 0;
        loop {
            let x = rand::thread_rng().gen_range(0..self.x_size());
            let y = rand::thread_rng().gen_range(0..self.y_size());

            if !self.limitations.contains(&(x, y)) {
                return (x, y);
            }
            loop_count += 1;

            assert!(loop_count < MAX_LOOP_COUNT);
        }
    }
}

struct GridPrinter;

impl GridPrinter {
    pub fn print(map: &Map) {
        let mut map_string = String::new();

        map_string.push_str(&Self::print_top_row(map.x_range, map.x_type));
        let coords = map.generate_coords();

        for (y_index, range_value) in (0..=(map.y_size())).enumerate() {
            let limitations_for_row = limitations_for_row(&map.limitations, range_value);

            let maybe_x_coord = {
                if coords.1 == y_index {
                    Some(coords.0)
                } else {
                    None
                }
            };

            map_string.push_str(&Self::print_value_row(
                Self::index_name(range_value + map.y_range.0, map.y_type),
                maybe_x_coord,
                limitations_for_row,
                map.x_size(),
            ));
            if y_index == map.y_size() {
                map_string.push_str(&Self::print_end_row(map.x_size()));
            } else {
                // map_string.push_str(&Self::print_value_separater_row(x_size));
            }
        }

        println!("{}", map_string);
    }

    // ref https://www.w3schools.com/charsets/ref_utf_box.asp
    const SPACE: char = ' ';
    const NEWLINE: char = '\n';
    const HORIZONTAL: char = '─';
    const VERTICAL: char = '│';
    const CORNER_TOP_RIGHT: char = '┐';
    const CORNER_TOP_LEFT: char = '┌';
    const CORNER_BOTTOM_RIGHT: char = '┘';
    const CORNER_BOTTOM_LEFT: char = '└';
    const T: char = '┬';
    const T_90: char = '┤';
    const T_180: char = '┴';
    const T_270: char = '├';
    const CROSS: char = '┼';
    const DASH: char = '-';
    const LIMITED: char = 'o';
    const COORDINATE: char = 'X';

    const PLACEHOLDER: char = '/';

    fn index_name(index: usize, range_type: RangeType) -> char {
        match range_type {
            RangeType::Alpha => index as u8 as char,
            RangeType::Numeric => char::from_digit(index as u32, 10).unwrap_or(Self::PLACEHOLDER),
        }
    }

    fn print_top_row(range: Range, range_type: RangeType) -> String {
        let mut row = String::new();
        let range_size = range.1 - range.0 + 1; // inclusive range

        row.push(Self::SPACE);
        row.push(Self::SPACE);
        for range_value in range.0..=range.1 {
            row.push(Self::index_name(range_value, range_type));
            row.push(Self::SPACE);
        }
        row.push(Self::NEWLINE);

        row.push(Self::SPACE);
        row.push(Self::CROSS);

        for i in 0..range_size {
            row.push(Self::HORIZONTAL);
            if i == range_size {
                row.push(Self::T_90);
            } else {
                row.push(Self::CROSS);
            }
        }
        row.push(Self::NEWLINE);
        row
    }

    fn print_value_row(
        leading_char: char,
        x_coord: Option<usize>,
        limitations_for_row: Vec<usize>,
        x_size: usize,
    ) -> String {
        let mut row = String::new();
        let horizontal_width = x_size * 2 + 1;
        let row_grid_count = horizontal_width / 2;

        row.push(leading_char);
        row.push(Self::VERTICAL);

        for i in 0..=row_grid_count {
            if limitations_for_row.contains(&i) {
                row.push(Self::LIMITED);
            } else if x_coord.is_none() {
                row.push(Self::SPACE);
            } else {
                let coord = x_coord.unwrap();
                if coord == i {
                    row.push(Self::COORDINATE);
                } else {
                    row.push(Self::SPACE);
                }
            }

            row.push(Self::VERTICAL);
        }
        row.push(Self::NEWLINE);
        row
    }

    fn print_value_separater_row(x_size: usize) -> String {
        let mut row = String::new();
        row.push(Self::SPACE);
        row.push(Self::T_270);
        let horizontal_width = x_size * 2 + 1;
        let row_grid_count = horizontal_width / 2;
        for i in 0..row_grid_count {
            row.push(Self::HORIZONTAL);
            if i + 1 == row_grid_count {
                row.push(Self::T_90);
            } else {
                row.push(Self::CROSS);
            }
        }
        row.push(Self::NEWLINE);
        row
    }

    fn print_end_row(x_size: usize) -> String {
        let mut row = String::new();
        row.push(Self::SPACE);

        for _ in 0..=x_size {
            row.push(Self::T_180);
            row.push(Self::HORIZONTAL);
        }
        row.push(Self::T_180);
        row.push(Self::NEWLINE);
        row
    }
}

fn limitations_for_row(limitations: &[Range], selected_row: usize) -> Vec<usize> {
    limitations
        .iter()
        .filter(|(_column, row)| *row == selected_row)
        .map(|pair| pair.0)
        .collect()
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

    GridPrinter::print(&map);
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
