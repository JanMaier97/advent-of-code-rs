use crate::common::math_2d::Grid;

use anyhow::{Context, Result};

pub fn parse_grid<T, F>(input: &str, mapper: F) -> Result<Grid<T>>
where
    F: Fn(char) -> Result<T>,
{
    let mut values = Vec::new();
    for (row_idx, line) in input.lines().enumerate() {
        let mut mapped_row = Vec::new();
        for (col_idx, char) in line.chars().enumerate() {
            let mapped_value = mapper(char).with_context(|| {
                format!("Failed parsing grid in row {} column {}", row_idx, col_idx)
            })?;
            mapped_row.push(mapped_value);
        }
        values.push(mapped_row);
    }

    Grid::from_raw_values(values)
}
