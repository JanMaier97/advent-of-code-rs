use std::collections::{HashMap, HashSet};

use crate::common::{
    math_2d::{Grid, Point, PointIdx, Vec2},
    parsing::parse_grid,
};

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Empty,
    End,
    Start,
}

struct Input {
    grid: Grid<Tile>,
    path: Vec<Point<i32>>,
}

fn count_shortcuts_by_time_saved(input: &Input) -> HashMap<usize, usize> {
    let shortcuts = find_shortcuts(&input.grid, &input.path);

    let mut saved_count = HashMap::new();

    for (start, end) in shortcuts {
        let start_idx = input.path.iter().position(|p| *p == start).unwrap();
        let end_idx = input.path.iter().position(|p| *p == end).unwrap();
        let time_saved = start_idx.max(end_idx) - end_idx.min(start_idx) - 2;

        saved_count
            .entry(time_saved)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    saved_count
}

fn find_shortcuts(grid: &Grid<Tile>, path: &[Point<i32>]) -> HashSet<(Point<i32>, Point<i32>)> {
    let mut starting_points = HashSet::new();

    for start in path.iter() {
        let shortcut_ends = vec![Vec2::UP, Vec2::LEFT, Vec2::RIGHT, Vec2::DOWN]
            .into_iter()
            .filter_map(|dir| check_shortcut_in_dir(*start, grid, dir))
            .collect_vec();

        for end in shortcut_ends {
            if !starting_points.contains(&(end, *start)) {
                starting_points.insert((*start, end));
            }
        }
    }

    starting_points
}

fn check_shortcut_in_dir(
    point: Point<i32>,
    grid: &Grid<Tile>,
    dir: Vec2<i32>,
) -> Option<Point<i32>> {
    let t1 = grid.get_at(point + dir);
    if t1.is_none() || t1.is_some_and(|t| *t != Tile::Wall) {
        return None;
    }

    let t2 = grid.get_at(point + dir * 2);
    if t2.is_none() || t2.is_some_and(|t| *t == Tile::Wall) {
        return None;
    }

    Some(point + dir * 2)
}

fn parse_input(input: &str) -> Result<Input> {
    let grid = parse_grid(input, map_to_tile)?;
    let path = get_path(&grid)?;

    Ok(Input { grid, path })
}

fn get_path(grid: &Grid<Tile>) -> Result<Vec<Point<i32>>> {
    let start = grid
        .find_tile_position(Tile::Start)
        .ok_or(anyhow!("Grid contains no start"))?;
    let end = grid
        .find_tile_position(Tile::End)
        .ok_or(anyhow!("Grid contains no end"))?;

    let mut visited = HashSet::from([start]);
    let mut path = vec![start];
    loop {
        let current_pos = path[path.len() - 1];
        if current_pos == end {
            break;
        }

        let next = get_next_points(current_pos)
            .into_iter()
            .filter(|&p| !visited.contains(&p) && grid.get_at(p).is_some_and(|t| *t != Tile::Wall))
            .collect_vec()
            .first()
            .cloned()
            .unwrap();

        visited.insert(next);
        path.push(next);
    }

    Ok(path)
}

fn get_next_points(point: Point<i32>) -> HashSet<Point<i32>> {
    HashSet::from([
        point + Vec2::UP,
        point + Vec2::DOWN,
        point + Vec2::RIGHT,
        point + Vec2::LEFT,
    ])
}

fn map_to_tile(char: char) -> Result<Tile> {
    let res = match char {
        'S' => Tile::Start,
        'E' => Tile::End,
        '.' => Tile::Empty,
        '#' => Tile::Wall,
        _ => bail!("invalid tile"),
    };

    Ok(res)
}
