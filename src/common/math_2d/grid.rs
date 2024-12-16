use anyhow::{bail, Result};

use super::Point;

pub struct Grid<T> {
    values: Vec<Vec<T>>,
    dim: Dimensions,
}

impl<T> Grid<T> {
    pub fn from_raw_values(values: Vec<Vec<T>>) -> Result<Self> {
        let dim = Dimensions::from_grid(&values)?;
        Ok(Self { values, dim })
    }

    pub fn dim(&self) -> Dimensions {
        self.dim
    }
}

pub trait PointIdx<T> {
    type Item;
    fn get_at(&self, point: Point<T>) -> Option<&Self::Item>;

    fn set_at(&mut self, point: Point<T>, value: Self::Item);
}

impl<T> PointIdx<i32> for Grid<T> {
    type Item = T;
    fn get_at(&self, point: Point<i32>) -> Option<&T> {
        if !self.dim.is_point_inside(point) {
            return None;
        }
        let x = point.x as usize;
        let y = point.y as usize;
        Some(&self.values[y][x])
    }

    fn set_at(&mut self, point: Point<i32>, value: T) {
        if !self.dim.is_point_inside(point) {
            return;
        }

        let x = point.x as usize;
        let y = point.y as usize;
        self.values[y][x] = value
    }
}

impl<T> PointIdx<u64> for Grid<T> {
    type Item = T;

    fn get_at(&self, point: Point<u64>) -> Option<&T> {
        let is_in_grid = self.dim.height > 0
            && self.dim.width > 0
            && point.x <= self.dim.width - 1
            && point.y <= self.dim.height - 1;

        if !is_in_grid {
            return None;
        }

        Some(&self.values[point.y as usize][point.x as usize])
    }

    fn set_at(&mut self, point: Point<u64>, value: T) {
        let is_in_grid = self.dim.height > 0
            && self.dim.width > 0
            && point.x <= self.dim.width - 1
            && point.y <= self.dim.height - 1;

        if !is_in_grid {
            return;
        }

        self.values[point.y as usize][point.x as usize] = value;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Dimensions {
    pub height: u64,
    pub width: u64,
}

impl Dimensions {
    fn new(height: u64, width: u64) -> Self {
        Self { height, width }
    }

    fn is_point_inside(&self, point: Point<i32>) -> bool {
        if self.height == 0 || self.width == 0 {
            return false;
        }

        if point.x < 0 || point.y < 0 {
            return false;
        }

        let max_x = self.width - 1;
        let max_y = self.height - 1;
        let x = point.x as u64;
        let y = point.y as u64;

        x <= max_x && y <= max_y
    }

    fn from_grid<T>(values: &[Vec<T>]) -> Result<Self> {
        if values.is_empty() {
            return Ok(Self::new(0, 0));
        }

        let width = values[0].len();
        let row_with_different_width = values
            .iter()
            .enumerate()
            .find(|(_, row)| row.len() != width)
            .map(|(idx, _)| idx);

        if let Some(row_idx) = row_with_different_width {
            bail!("Row {} has a different size than the first row", row_idx);
        }

        Ok(Dimensions::new(values.len().try_into()?, width.try_into()?))
    }
}
