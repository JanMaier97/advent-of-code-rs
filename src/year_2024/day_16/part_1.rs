use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use macros::aoc_solver;

use crate::{
    common::{
        math_2d::{Grid, Point, PointIdx, Vec2},
        parsing::parse_grid,
    },
    MyResult,
};

type RaceResult = (u64, HashMap<(Point<i32>, Vec2<i32>), u64>);

#[derive(Debug, Hash, Eq, PartialEq)]
struct NextPoint {
    pos: Point<i32>,
    dir: Vec2<i32>,
    cost: u64,
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empyt,
    Wall,
    Start,
    End,
}

struct Map {
    grid: Grid<Tile>,
    start_pos: Point<i32>,
}

#[aoc_solver(2024, 16, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let map = parse_input(input)?;

    let (base_cost, visited) = find_cost_for_shortest_path(&map.grid, map.start_pos);
    let (score, _) = find_lowest_cost(
        &map.grid,
        &visited,
        map.start_pos,
        Vec2::RIGHT,
        0,
        base_cost,
    );
    Ok(score)
}

#[aoc_solver(2024, 16, 2, super::INPUT)]
fn solve_part_2(input: &str) -> MyResult<u64> {
    let map = parse_input(input)?;

    let (base_cost, visited) = find_cost_for_shortest_path(&map.grid, map.start_pos);
    let (_, path) = find_lowest_cost(
        &map.grid,
        &visited,
        map.start_pos,
        Vec2::RIGHT,
        0,
        base_cost,
    );

    Ok(path.len().try_into()?)
}

fn parse_input(input: &str) -> Result<Map> {
    let grid = parse_grid(input, map_char_to_tile)?;
    let start_pos = grid
        .find_tile_position(Tile::Start)
        .ok_or(anyhow!("Could not find the start position in the grid."))?;
    Ok(Map { grid, start_pos })
}

fn find_lowest_cost(
    grid: &Grid<Tile>,
    visited: &HashMap<(Point<i32>, Vec2<i32>), u64>,
    current_pos: Point<i32>,
    current_dir: Vec2<i32>,
    current_cost: u64,
    best_cost_yet: u64,
) -> (u64, HashSet<Point<i32>>) {
    if current_cost > best_cost_yet {
        return (u64::MAX, HashSet::new());
    }

    if let Some(existing_cost) = visited.get(&(current_pos, current_dir)) {
        if *existing_cost < current_cost {
            return (u64::MAX, HashSet::new());
        }
    }

    // point hasn't been visted yet or current path is more efficient
    let mut updated_visitied = visited.clone();
    updated_visitied.insert((current_pos, current_dir), current_cost);

    let current_tile = *grid.get_at(current_pos).unwrap();
    if current_tile == Tile::End {
        return (current_cost, HashSet::from([current_pos]));
    }

    let next_points = get_next_points(grid, current_pos, current_dir);
    if next_points.is_empty() {
        return (u64::MAX, HashSet::new());
    }

    let mut best_cost = best_cost_yet;
    let mut path_points = HashSet::new();
    let mut found_an_end = false;
    for next in next_points {
        let updated_cost = current_cost + next.cost;
        let (next_cost, next_paths) = find_lowest_cost(
            grid,
            &updated_visitied,
            next.pos,
            next.dir,
            updated_cost,
            best_cost,
        );

        if next_cost < u64::MAX {
            found_an_end = true;
        }

        if next_cost  <= best_cost {
            path_points.extend(next_paths);
            path_points.insert(current_pos);
        }

        best_cost = best_cost.min(next_cost);
    }

    if !found_an_end {
        return (u64::MAX, HashSet::new());
    }

    (best_cost, path_points)
}

fn get_next_points(grid: &Grid<Tile>, pos: Point<i32>, current_dir: Vec2<i32>) -> Vec<NextPoint> {
    let mut points = Vec::new();

    let next_pos = pos + current_dir;
    if *grid.get_at(next_pos).unwrap() != Tile::Wall {
        points.push(NextPoint {
            pos: next_pos,
            dir: current_dir,
            cost: 1,
        });
    }

    let (left_dir, right_dir) = get_next_dir(current_dir);
    let left_pos = pos + left_dir;
    if *grid.get_at(left_pos).unwrap() != Tile::Wall {
        points.push(NextPoint {
            pos: left_pos,
            dir: left_dir,
            cost: 1 + 1000,
        });
    }

    let right_pos = pos + right_dir;
    if *grid.get_at(right_pos).unwrap() != Tile::Wall {
        points.push(NextPoint {
            pos: right_pos,
            dir: right_dir,
            cost: 1 + 1000,
        });
    }

    if points.is_empty() {
        return Vec::new();
    }

    points
}

fn get_next_dir(dir: Vec2<i32>) -> (Vec2<i32>, Vec2<i32>) {
    match dir {
        Vec2::RIGHT => (Vec2::UP, Vec2::DOWN),
        Vec2::UP => (Vec2::LEFT, Vec2::RIGHT),
        Vec2::LEFT => (Vec2::DOWN, Vec2::UP),
        Vec2::DOWN => (Vec2::RIGHT, Vec2::LEFT),
        _ => panic!("invalid direction"),
    }
}

fn map_char_to_tile(char: char) -> Result<Tile> {
    match char {
        '#' => Ok(Tile::Wall),
        '.' => Ok(Tile::Empyt),
        'S' => Ok(Tile::Start),
        'E' => Ok(Tile::End),
        _ => bail!("found invalid tile character '{}'", char),
    }
}

#[allow(dead_code)]
fn print_map(grid: &Grid<Tile>, pos: Point<i32>, path: &HashSet<Point<i32>>) {
    for y in 0..grid.dim().height {
        for x in 0..grid.dim().width {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            if pos == point {
                print!("S");
            } else if path.contains(&point) {
                print!("O")
            } else {
                let tile = grid.get_at(point).unwrap();
                let char = match tile {
                    Tile::Empyt => '.',
                    Tile::Wall => '#',
                    Tile::Start => '.',
                    Tile::End => 'E',
                };
                print!("{}", char);
            }
        }
        println!()
    }
}

fn find_cost_for_shortest_path(
    grid: &Grid<Tile>,
    start_pos: Point<i32>,
) -> RaceResult {
    let mut visited = HashMap::new();
    let to_visit = HashSet::from([NextPoint {
        pos: start_pos,
        dir: Vec2::RIGHT,
        cost: 0,
    }]);
    let s = bfs_sortest_path(grid, &mut visited, to_visit);

    (s, visited)
}

fn bfs_sortest_path(
    grid: &Grid<Tile>,
    visited: &mut HashMap<(Point<i32>, Vec2<i32>), u64>,
    to_visit: HashSet<NextPoint>,
) -> u64 {
    let neighbours = to_visit
        .iter()
        .flat_map(|n| {
            get_next_points(grid, n.pos, n.dir)
                .into_iter()
                .map(|n2| NextPoint {
                    pos: n2.pos,
                    dir: n2.dir,
                    cost: n.cost + n2.cost,
                })
        })
        .collect_vec();

    let mut next_to_visit: HashSet<NextPoint> = HashSet::new();
    for neighbour in neighbours {
        if *grid.get_at(neighbour.pos).unwrap() == Tile::End {
            return neighbour.cost;
        }

        if let Some(&existing_cost) = visited.get(&(neighbour.pos, neighbour.dir)) {
            if existing_cost <= neighbour.cost {
                continue;
            }
        }

        next_to_visit.insert(neighbour);
    }
    for n in to_visit {
        visited.insert((n.pos, n.dir), n.cost);
    }

    bfs_sortest_path(grid, visited, next_to_visit)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_1() {
        let result = super::solve(include_str!("example_1.txt")).unwrap();
        assert_eq!(result, 7036);
    }

    #[test]
    fn solve_example_2() {
        let result = super::solve(include_str!("example_2.txt")).unwrap();
        assert_eq!(result, 11048);
    }

    #[test]
    fn solve_part_2_example_1() {
        let result = super::solve_part_2(include_str!("example_1.txt")).unwrap();
        assert_eq!(result, 45);
    }

    #[test]
    fn solve_part_2_example_2() {
        let result = super::solve_part_2(include_str!("example_2.txt")).unwrap();
        assert_eq!(result, 64);
    }
}
