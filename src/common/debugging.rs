use std::collections::HashSet;

use super::math_2d::{Grid, Point};

pub fn print_grid<T, F>(grid: &Grid<T>, mapper: F, points_to_highlight: HashSet<Point<i32>>)
where
    F: Fn(&T) -> char,
{
    for y in 0..grid.dim().height {
        for x in 0..grid.dim().width {
            let point = Point::new(x as i32, y as i32);
            if points_to_highlight.contains(&point) {
                print!("O");
            } else {
                print!("{}", mapper(&grid[point]));
            }
        }
        println!();
    }
}
