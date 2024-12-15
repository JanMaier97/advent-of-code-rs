use crate::common::{
    math_2d::{Grid, Point, PointIdx, Vec2},
    parsing::parse_grid,
};

use anyhow::{bail, Result};
use itertools::Itertools;

mod part_1;

const INPUT: &str = include_str!("input.txt");

struct Map {
    grid: Grid<Tile>,
    robot_pos: Point<i32>,
    directions: Vec<Vec2<i32>>,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
    Robot,
    Box,
}

impl Tile {
    fn try_from_char(char: char) -> Result<Tile> {
        match char {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Empty),
            '@' => Ok(Tile::Robot),
            'O' => Ok(Tile::Box),
            other => bail!("Encountered an invalid tile character: '{other}'"),
        }
    }
}

fn parse_input(input: &str) -> Result<Map> {
    let blocks = input.split("\r\n\r\n").collect_vec();

    if blocks.len() != 2 {
        bail!(
            "Input invalid: expected 2 seprate blocks, but got {}",
            blocks.len()
        );
    }

    let grid = parse_grid(blocks[0], Tile::try_from_char)?;
    let find_robot_position = find_robot_position(&grid);
    let pos = find_robot_position?;
    let directions = parse_directions(blocks[1])?;

    Ok(Map {
        grid,
        robot_pos: pos,
        directions,
    })
}

fn find_robot_position(grid: &Grid<Tile>) -> Result<Point<i32>> {
    for row in 0..grid.dim().height {
        for col in 0..grid.dim().width {
            let point = Point::new(row, col);

            let Some(tile) = grid.get_at(point) else {
                continue;
            };

            if *tile == Tile::Robot {
                let x = col.try_into()?;
                let y = row.try_into()?;
                return Ok(Point::new(x, y));
            }
        }
    }

    bail!("Input is missing the robot position");
}

fn parse_directions(input: &str) -> Result<Vec<Vec2<i32>>> {
    let mut values: Vec<Vec2<i32>> = Vec::new();

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            let dir = match char {
                '>' => Vec2::right(),
                '<' => Vec2::left(),
                '^' => Vec2::up(),
                'v' => Vec2::down(),
                _ => bail!(
                    "Invalid direction '{}' in row {} column {}",
                    char,
                    row_idx,
                    col_idx
                ),
            };
            values.push(dir);
        }
    }

    Ok(values)
}

fn find_box_positions(grid: &Grid<Tile>) -> Vec<Point<i32>> {
    let dim = grid.dim();
    let mut points = Vec::new();

    for y in 0..dim.height {
        for x in 0..dim.width {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            let Some(tile) = grid.get_at(point) else {
                continue;
            };

            if *tile == Tile::Box {
                points.push(point);
            }
        }
    }

    points
}

fn get_score_gps(point: Point<i32>) -> u64 {
    point.y as u64 * 100 + point.x as u64
}
