use std::collections::HashSet;

use macros::aoc_solver;

use anyhow::Result;

use crate::common::{
    math_2d::{Dimensions, Grid, Point, PointIdx, Vec2},
    parsing,
};

static INPUT: &str = include_str!("input.txt");

#[derive(PartialEq)]
enum Tile {
    Empty,
    Box,
}

#[aoc_solver(2025, 4, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    let grid = parse_grid(input)?;
    let boxes = get_accesible_boxes(&grid);
    Ok(boxes.len().to_string())
}

#[aoc_solver(2025, 4, 2, INPUT)]
fn solve_part_2(input: &str) -> Result<String> {
    let mut grid = parse_grid(input)?;
    let mut count = 0;
    loop {
        let boxes = get_accesible_boxes(&grid);
        count += boxes.len();
        if boxes.len() == 0 {
            break;
        }

        for point in boxes {
            grid.set_at(point, Tile::Empty);
        }
    }

    Ok(count.to_string())
}

fn parse_grid(input: &str) -> Result<Grid<Tile>> {
    let mapper = |c| {
        if c == '.' {
            Ok(Tile::Empty)
        } else {
            Ok(Tile::Box)
        }
    };
    let grid = parsing::parse_grid(input, mapper)?;

    Ok(grid)
}

fn get_accesible_boxes(grid: &Grid<Tile>) -> HashSet<Point<i32>> {
    let mut points = HashSet::new();
    for x in 0..grid.dim().width {
        for y in 0..grid.dim().height {
            let point = Point::new(x as i32, y as i32);

            if grid.get_at(point).unwrap() != &Tile::Box {
                continue;
            }

            let close_boxes = get_sourrunding_tiles(point, grid.dim())
                .iter()
                .map(|&p| grid.get_at(p).unwrap())
                .filter(|t| **t == Tile::Box)
                .count();

            if close_boxes < 4 {
                points.insert(point);
            }
        }
    }

    points
}

fn get_sourrunding_tiles(pos: Point<i32>, dim: Dimensions) -> HashSet<Point<i32>> {
    let points = HashSet::from([
        pos - Vec2::UP,
        pos - Vec2::DOWN,
        pos - Vec2::RIGHT,
        pos - Vec2::LEFT,
        pos - Vec2::new(-1, -1),
        pos - Vec2::new(1, -1),
        pos - Vec2::new(-1, 1),
        pos - Vec2::new(1, 1),
    ]);

    points
        .into_iter()
        .filter(|p| p.x >= 0 && p.y >= 0 && (p.x as u64) < dim.width && (p.y as u64) < dim.height)
        .collect::<HashSet<Point<i32>>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve_part_1(include_str!("example.txt")).unwrap();
        assert_eq!(result, "13");
    }

    #[test]
    fn solve_example_part_2() {
        let result = super::solve_part_2(include_str!("example.txt")).unwrap();
        assert_eq!(result, "43");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "1457");
    }

    #[test]
    fn solve_part_2() {
        let result = super::solve_part_2(super::INPUT).unwrap();
        assert_eq!(result, "8310");
    }
}
