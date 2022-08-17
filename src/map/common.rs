pub type Range = (usize, usize);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RangeType {
    Alpha,
    Numeric,
}

#[derive(PartialEq, Debug)]
pub struct Map {
    pub name: String,
    pub x_type: RangeType,
    pub y_type: RangeType,
    pub x_range: Range,
    pub y_range: Range,
    pub limitations: Vec<Range>,
}
