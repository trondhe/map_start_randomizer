use std::fs::File;
use std::io::Read;

use serde::Deserialize;
use serde::Serialize;

use super::RangeType;

use super::Map;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) enum MapPointValue {
    Alpha(char),
    Numeric(isize),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct MapPoint {
    pub point: (MapPointValue, MapPointValue),
}

impl MapPoint {
    pub(crate) fn to_tuple(&self) -> (isize, isize) {
        let first = match self.point.0 {
            MapPointValue::Alpha(value) => (value as u8 - 65) as isize,
            MapPointValue::Numeric(value) => value,
        };
        let second = match self.point.1 {
            MapPointValue::Alpha(value) => (value as u8 - 65) as isize,
            MapPointValue::Numeric(value) => value,
        };
        (first, second)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct MapJson {
    pub(crate) name: String,
    pub(crate) start: MapPoint,
    pub(crate) end: MapPoint,
    pub(crate) limitations: Vec<MapPoint>,
}

impl From<MapJson> for Map {
    fn from(map_json: MapJson) -> Self {
        Self {
            name: map_json.name.clone(),
            x_type: map_json.x_range_type(),
            y_type: map_json.y_range_type(),
            x_range: map_json.x_range(),
            y_range: map_json.y_range(),
            limitations: map_json.limitations().clone(),
        }
    }
}

impl MapJson {
    pub(crate) fn x_range_type(&self) -> RangeType {
        match self.start.point.0 {
            MapPointValue::Alpha(_) => RangeType::Alpha,
            MapPointValue::Numeric(_) => RangeType::Numeric,
        }
    }

    pub(crate) fn y_range_type(&self) -> RangeType {
        match self.start.point.1 {
            MapPointValue::Alpha(_) => RangeType::Alpha,
            MapPointValue::Numeric(_) => RangeType::Numeric,
        }
    }

    pub(crate) fn x_range(&self) -> (isize, isize) {
        let start = match self.start.point.0 {
            MapPointValue::Alpha(value) => (value as u8 - 65) as isize,
            MapPointValue::Numeric(value) => value,
        };
        let end = match self.end.point.0 {
            MapPointValue::Alpha(value) => (value as u8 - 65) as isize,
            MapPointValue::Numeric(value) => value,
        };
        (start, end)
    }

    pub(crate) fn y_range(&self) -> (isize, isize) {
        let start = match self.start.point.1 {
            MapPointValue::Alpha(value) => (value as u8 - 65) as isize,
            MapPointValue::Numeric(value) => value,
        };
        let end = match self.end.point.1 {
            MapPointValue::Alpha(value) => (value as u8 - 65) as isize,
            MapPointValue::Numeric(value) => value,
        };
        (start, end)
    }

    pub(crate) fn limitations(&self) -> Vec<(isize, isize)> {
        let mut limitations = Vec::new();
        for limitation in &self.limitations {
            limitations.push(limitation.to_tuple());
        }
        limitations
    }
}

impl Default for MapJson {
    fn default() -> Self {
        Self {
            name: "generic".to_owned(),
            start: MapPoint {
                point: (MapPointValue::Alpha('A'), MapPointValue::Alpha('I')),
            },
            end: MapPoint {
                point: (MapPointValue::Alpha('H'), MapPointValue::Alpha('P')),
            },
            limitations: vec![],
        }
    }
}
pub(crate) fn read_file() -> Option<String> {
    let file = File::open("specialMaps.json");
    if file.is_err() {
        return None;
    }
    let mut file = file.unwrap();

    let mut data = String::new();
    let result = file.read_to_string(&mut data);
    if result.is_err() {
        println!("Could not read special maps into string");
        return None;
    }
    Some(data)
}

fn parse_maps(json_string: &str) -> Option<Vec<MapJson>> {
    let deserialized = serde_json::from_str(&json_string).unwrap_or_else(|val| {
        println!("Could not deserialize special maps with err: '{}'", val);
        return None;
    });
    return deserialized;
}

pub(crate) fn special_map(name: String) -> Option<Map> {
    let maps_string = include_str!("maps.json");
    let maps = parse_maps(maps_string)?;
    let map_json = maps.into_iter().find(|map| map.name == name)?;

    Some(Map::from(map_json))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty_is_none() {
        let empty_data = "".to_owned();
        let result = parse_maps(&empty_data);
        assert!(result.is_none());
    }
}
