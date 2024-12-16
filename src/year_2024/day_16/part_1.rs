
use std::{collections::{HashMap, HashSet}, u64};

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use macros::aoc_solver;

use crate::{common::{math_2d::{Grid, Point, PointIdx, Vec2}, parsing::parse_grid}, MyResult};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empyt,
    Wall,
    Start,
    End,
}

struct Map {
    grid: Grid<Tile>,
    start_pos: Point<i32>
}

#[aoc_solver(2024, 16, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let map = parse_input(input)?;
    
    let visited = HashMap::new();
    let score  = find_lowest_cost(&map.grid, &visited, map.start_pos, Vec2::RIGHT, 0, u64::MAX);
    Ok(score)
}

fn parse_input(input: &str) -> Result<Map> {
    let grid = parse_grid(input, map_char_to_tile)?;
    let start_pos = grid.find_tile_position(Tile::Start)
        .ok_or(anyhow!("Could not find the start position in the grid."))?;
    Ok(Map {
        grid, start_pos
    })
}

fn find_lowest_cost(grid: &Grid<Tile>, visited: &HashMap<Point<i32>, u64>, current_pos: Point<i32>, current_dir: Vec2<i32>, current_cost: u64, best_cost_yet: u64) -> u64 {
    // print_map(grid, current_pos, &Vec::new());
    // std::io::stdin().read_line(&mut String::new()).unwrap();

    if let Some(existing_cost) = visited.get(&current_pos) {
        if *existing_cost <= current_cost {
            // the point has already been reached with the same cost of less
            return current_cost;
        }
    }

    // point hasn't been visted yet or current path is more efficient

    let mut updated_visitied = visited.clone();
    updated_visitied.insert(current_pos, current_cost); 

    let current_tile = *grid.get_at(current_pos).unwrap();

    if current_tile == Tile::End {
        return current_cost;
    }

    let next_points = get_next_points(grid, visited, current_pos, current_dir);
    if next_points.is_empty() {
        return u64::MAX;
    }

    let mut best_cost = best_cost_yet;
    for next in next_points {
        let updated_cost = current_cost + next.cost;
        if updated_cost > best_cost {
            // println!("{:?} is too expensive\n\n", next.pos);
            // exit early if the next move already exceeds the lowest score yet
            return best_cost;
        }

        let next_cost = find_lowest_cost(grid, &updated_visitied, next.pos, next.dir, updated_cost, best_cost);
        best_cost = best_cost.min(next_cost);
    }

    best_cost
}

#[derive(Debug)]
struct NextPoint {
    pos: Point<i32>,
    dir: Vec2<i32>,
    cost: u64, 
}

fn get_next_points(grid: &Grid<Tile>, visited: &HashMap<Point<i32>, u64>, pos: Point<i32>, current_dir: Vec2<i32>) -> Vec<NextPoint> {
    let mut points = Vec::new();

    let next_pos = pos + current_dir;
    if *grid.get_at(next_pos).unwrap() != Tile::Wall {
        points.push(NextPoint {
            pos: next_pos, dir: current_dir, cost: 1,
        });
    }

    let (left_dir, right_dir) = get_next_dir(current_dir);
    let left_pos = pos + left_dir;
    if *grid.get_at(left_pos).unwrap() != Tile::Wall {
        points.push(NextPoint {
            pos: left_pos, dir: left_dir, cost: 1 + 1000,
        });
    }

    let right_pos = pos + right_dir;
    if *grid.get_at(right_pos).unwrap() != Tile::Wall {
        points.push(NextPoint {
            pos: right_pos, dir: right_dir, cost: 1 + 1000,
        });
    }

    if points.len() == 0 {
        // println!("{:?} is a dead end", pos);
        return Vec::new();
    }

    let filtered = points
        .into_iter()
        .filter(|p| !visited.contains_key(&p.pos))
        .collect_vec();

    if filtered.is_empty() {
        // println!("{:?} ends in a loop", pos);
        return Vec::new();
    }

    // println!("next positions to try: {:?}", filtered);

    filtered
}


fn get_next_dir(dir: Vec2<i32>) -> (Vec2<i32>, Vec2<i32>) {
    match dir {
        Vec2::RIGHT => (Vec2::UP, Vec2::DOWN),
        Vec2::UP => (Vec2::LEFT, Vec2::RIGHT),
        Vec2::LEFT => (Vec2::DOWN, Vec2::UP),
        Vec2::DOWN => (Vec2::RIGHT, Vec2::LEFT),
        _ => panic!("invalid direction")
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

fn print_map(grid: &Grid<Tile>, pos: Point<i32>, path: &[Point<i32>]) {
    println!();
    println!();
    for y in 0..grid.dim().height {
        for x in 0..grid.dim().width {
            let point = Point{x: x as i32, y: y as i32};
            if pos == point {
                print!("S");
            } else if path.contains(&point) {
                print!("$")
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
}