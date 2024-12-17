use itertools::Itertools;
use macros::aoc_solver;

use crate::{
    common::math_2d::{Grid, Point, PointIdx, Vec2},
    year_2024::day_15::{find_tile_position, parse_input, Map},
    MyResult,
};

use super::{find_box_positions, get_score_gps, print_grid, Tile};

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DoubleTile {
    Empty,
    Wall,
    LeftBox,
    RightBox,
    Robot,
}

#[aoc_solver(2024, 15, 2, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let map = parse_input(input)?;
    let grid = enlarge_map(&map.grid)?;
    let mut map = Map {
        directions: map.directions,
        robot_pos: find_tile_position(&grid, DoubleTile::Robot).unwrap(),
        grid,
    };

    apply_movement(&mut map);

    let points = find_box_positions(&map.grid, |&tile| tile == DoubleTile::LeftBox);
    let sum = points.iter().map(|p| get_score_gps(*p)).sum();
    Ok(sum)
}

fn apply_movement(map: &mut Map<DoubleTile>) {
    for (iteration, dir) in map.directions.iter().cloned().enumerate() {
        let points_to_move = get_points_to_move(map.robot_pos, &map.grid, dir);
        for point in points_to_move.iter().cloned() {
            let tile_to_move = *map.grid.get_at(point).unwrap();
            map.grid.set_at(point + dir, tile_to_move);
            map.grid.set_at(point, DoubleTile::Empty);
        }

        if !points_to_move.is_empty() {
            map.robot_pos += dir;
        }
    }
}

fn get_points_to_move(
    robot_pos: Point<i32>,
    grid: &Grid<DoubleTile>,
    direction: Vec2<i32>,
) -> Vec<Point<i32>> {
    if direction.x != 0 {
        return get_horizontal_points_to_move(robot_pos, grid, direction)
            .into_iter()
            .rev()
            .collect_vec();
    }

    get_vertical_points_to_move(robot_pos, grid, direction)
        .into_iter()
        .collect_vec()
}

fn get_vertical_points_to_move(
    point_to_move: Point<i32>,
    grid: &Grid<DoubleTile>,
    direction: Vec2<i32>,
) -> Vec<Point<i32>> {
    let point_in_front = point_to_move + direction;
    let tile_in_front = *grid.get_at(point_in_front).unwrap();

    match tile_in_front {
        DoubleTile::Empty => vec![point_to_move],
        DoubleTile::Wall => Vec::new(),
        DoubleTile::LeftBox => {
            get_vertical_points_to_move_for_box(point_to_move, point_in_front, grid, direction)
        }
        DoubleTile::RightBox => get_vertical_points_to_move_for_box(
            point_to_move,
            point_in_front + Vec2::LEFT,
            grid,
            direction,
        ),
        DoubleTile::Robot => panic!("encountered a second robot"),
    }
}

fn get_vertical_points_to_move_for_box(
    point_to_move: Point<i32>,
    left_box_position: Point<i32>,
    grid: &Grid<DoubleTile>,
    direction: Vec2<i32>,
) -> Vec<Point<i32>> {
    let mut left_side = get_vertical_points_to_move(left_box_position, grid, direction);
    if left_side.is_empty() {
        return Vec::new();
    }

    let right_box_position = left_box_position + Vec2::RIGHT;
    let right_side = get_vertical_points_to_move(right_box_position, grid, direction);
    if right_side.is_empty() {
        return Vec::new();
    }

    let right_side = right_side
        .iter()
        .cloned()
        .filter(|p| !left_side.contains(p))
        .collect_vec();

    left_side.extend(right_side);
    left_side.push(point_to_move);
    left_side
}

fn get_horizontal_points_to_move(
    robot_pos: Point<i32>,
    grid: &Grid<DoubleTile>,
    direction: Vec2<i32>,
) -> Vec<Point<i32>> {
    let mut points = Vec::new();
    let mut current_pos = robot_pos;
    loop {
        let tile = *grid.get_at(current_pos).unwrap();

        if tile == DoubleTile::Wall {
            // nothing to move
            return Vec::new();
        }

        if tile == DoubleTile::Empty {
            break;
        }

        points.push(current_pos);
        current_pos += direction;
    }

    points
}

fn enlarge_map(grid: &Grid<Tile>) -> Result<Grid<DoubleTile>> {
    let mut rows: Vec<Vec<DoubleTile>> = Vec::new();

    for y in 0..grid.dim().height {
        let mut current_row = Vec::new();
        for x in 0..grid.dim().width {
            let point = Point::new(x, y);
            let tile = *grid.get_at(point).unwrap();
            current_row.push(match tile {
                Tile::Empty => DoubleTile::Empty,
                Tile::Wall => DoubleTile::Wall,
                Tile::Robot => DoubleTile::Robot,
                Tile::Box => DoubleTile::LeftBox,
            });
            current_row.push(match tile {
                Tile::Empty => DoubleTile::Empty,
                Tile::Wall => DoubleTile::Wall,
                Tile::Robot => DoubleTile::Empty,
                Tile::Box => DoubleTile::RightBox,
            });
        }
        rows.push(current_row);
    }

    Grid::from_raw_values(rows)
}

#[allow(dead_code)]
fn print_double_grid(grid: &Grid<DoubleTile>) {
    print_grid(grid, map_tile);
    println!();
}

#[allow(dead_code)]
fn map_tile(tile: &DoubleTile) -> char {
    match tile {
        DoubleTile::Empty => '.',
        DoubleTile::Wall => '#',
        DoubleTile::LeftBox => '[',
        DoubleTile::RightBox => ']',
        DoubleTile::Robot => '@',
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 9021);
    }

    #[test]
    fn solve_small_example_2() {
        let result = super::solve(include_str!("small_example_2.txt")).unwrap();
        assert_eq!(result, 618);
    }
}
