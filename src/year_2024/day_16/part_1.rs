
use std::{alloc::GlobalAlloc, collections::{HashMap, HashSet}, u64};

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use macros::aoc_solver;
use rayon::iter::{empty, Empty};

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

    let next_points: Vec<NextPoint> = get_next_points(grid, current_pos, current_dir);
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

fn get_next_points(grid: &Grid<Tile>, pos: Point<i32>, current_dir: Vec2<i32>) -> Vec<NextPoint> {
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


    points
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

fn find_shortes_crossing_path(map: HashMap<Point<i32>, HashSet<Point<i32>>>, start: Point<i32>, end: Point<i32>) -> Vec<Point<i32>> {
    let mut visited: HashSet<Point<i32>> = HashSet::new();
    let mut paths: Vec<Vec<Point<i32>>> = vec![vec![start]];


    loop {
        let Some(path) = paths.pop() else {
            break;
        };

        let current_pos = *path.last().unwrap();

        if visited.contains(&current_pos) {
            continue;
        }

        let neighbours = map.get(&current_pos).unwrap();

        for neighbour in neighbours {
            let mut new_path = path.clone();
            new_path.push(current_pos);
            paths.push(new_path);

            neighbour == end {
                return new_path;
            };


            
            visited.insert(current_pos);
        }

    }


}



fn collect_crossings(grid: &Grid<Tile>) -> HashMap<Point<i32>, HashSet<Point<i32>>> {
    let crossings = find_crossings(grid);

    let mut mapping = HashMap::new();
    for point in crossings.iter().cloned() {
        let reachable = find_reachable_crossings(grid, &crossings, point);
        mapping.insert(point, reachable);

    }

    mapping
}

fn find_reachable_crossings(grid: &Grid<Tile>, crossings: &HashSet<Point<i32>>, start_pos: Point<i32>) -> HashSet<Point<i32>> {
    let s1 = find_reachable_crossings_in_dir(grid, crossings, start_pos, Vec2::UP);
    let s2 = find_reachable_crossings_in_dir(grid, crossings, start_pos, Vec2::DOWN);
    let s3 = find_reachable_crossings_in_dir(grid, crossings, start_pos, Vec2::LEFT);
    let s4 = find_reachable_crossings_in_dir(grid, crossings, start_pos, Vec2::RIGHT);

    let mut result = HashSet::from(s1);
    result.extend(s2);
    result.extend(s3);
    result.extend(s4);

    result
}

fn find_reachable_crossings_in_dir(grid: &Grid<Tile>, crossings: &HashSet<Point<i32>>, start_pos: Point<i32>, direction: Vec2<i32>) -> HashSet<Point<i32>> {
    let mut reachable_crossings = HashSet::new();
    let mut current_pos = start_pos;
    loop {
        current_pos += direction;
        if crossings.contains(&current_pos) {
            reachable_crossings.insert(current_pos);
        }

        let tile = *grid.get_at(current_pos).unwrap();
        if tile == Tile::Wall {
            break;
        }
    }

    reachable_crossings
}


fn find_crossings(grid: &Grid<Tile>) -> HashSet<Point<i32>> {
    let mut crossings = HashSet::new();
    for y in 0..grid.dim().height {
        for x in 0..grid.dim().width {
            let point: Point<i32> = Point{x: x as i32, y: y as i32};
            let tile = *grid.get_at(point).unwrap();
            if tile == Tile::Wall {
                continue;
            }

            if tile == Tile::Start || tile == Tile::End || is_crossing(grid, point) {
                crossings.insert(point);
            }
        }
    }

    crossings
}

fn is_crossing(grid: &Grid<Tile>, pos: Point<i32>) -> bool {
    let positions = vec![
        pos + Vec2::UP,
        pos + Vec2::DOWN,
        pos + Vec2::RIGHT,
        pos + Vec2::LEFT,
    ];
    let neighbours = positions.iter().map(|p| *grid.get_at(*p).unwrap()).collect_vec();

    let free_tiles = neighbours.iter().filter(|t| **t != Tile::Wall).count();
    if free_tiles == 1 {
        return false;
    }

    if free_tiles > 3 {
        return true;
    }

    // crossing only exists for 2 free tiles
    // if they are not opsosite of each other
    if neighbours[0] != Tile::Wall && neighbours[1] != Tile::Wall {
        return false;
    }

    if neighbours[2] != Tile::Wall && neighbours[3] != Tile::Wall {
        return false;
    }

    return true;
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