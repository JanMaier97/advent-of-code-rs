use macros::aoc_solver;

use crate::common::math_2d::{Grid, Point, PointIdx, Vec2};

use super::{find_box_positions, get_score_gps, parse_input, Map, Tile};

use anyhow::Result;

#[aoc_solver(2024, 15, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let mut map = parse_input(input)?;

    apply_movement(&mut map);

    let points = find_box_positions(&map.grid, |tile| *tile == Tile::Box);
    let sum: u64 = points.iter().map(|point| get_score_gps(*point)).sum();

    Ok(sum.to_string())
}

fn apply_movement(map: &mut Map<Tile>) {
    for dir in map.directions.iter() {
        let Some(mut free_tile) = find_free_space_in_direction(&map.grid, map.robot_pos, *dir)
        else {
            continue;
        };

        let mut next_tile = free_tile - *dir;

        while free_tile != map.robot_pos {
            let value = map.grid.get_at(next_tile).unwrap();
            map.grid.set_at(free_tile, *value);
            free_tile = next_tile;
            next_tile = free_tile - *dir;
        }
        map.grid.set_at(map.robot_pos, Tile::Empty);
        map.robot_pos = free_tile + *dir;
    }
}

fn find_free_space_in_direction(
    grid: &Grid<Tile>,
    current_pos: Point<i32>,
    dir: Vec2<i32>,
) -> Option<Point<i32>> {
    let mut pos = current_pos;
    loop {
        pos += dir;
        let tile = grid.get_at(pos)?;
        match tile {
            Tile::Wall => return None,
            Tile::Empty => return Some(pos),
            Tile::Box => continue,
            Tile::Robot => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "10092");
    }

    #[test]
    fn solve_small_example() {
        let result = super::solve(include_str!("small_example.txt")).unwrap();
        assert_eq!(result, "2028");
    }
}
