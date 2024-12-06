use std::collections::HashSet;

use crate::MyResult;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Dimensions {
    height: usize,
    width: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Guard {
    pos: Point,
    dir: Direction,
}

#[derive(Clone)]
struct Map {
    obstacles: HashSet<Point>,
    dim: Dimensions,
}

fn get_next_position(current: &Guard, map: &Map) -> Option<Guard> {
    let (x, y) = match current.dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };

    let next_x = current.pos.x as i32 + x;
    let next_y = current.pos.y as i32 + y;
    if next_x < 0 || next_y < 0 {
        return None;
    }

    let next_pos = Point {
        x: next_x as usize,
        y: next_y as usize,
    };
    if next_pos.x >= map.dim.width || next_pos.y >= map.dim.height {
        return None;
    }

    if map.obstacles.contains(&next_pos) {
        let next_direction = match current.dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
        let guard = Guard {
            pos: current.pos,
            dir: next_direction,
        };
        return get_next_position(&guard, map);
    }

    Some(Guard {
        pos: next_pos,
        dir: current.dir,
    })
}

fn determine_guard_path(guard: Guard, map: &Map) -> Vec<Point> {
    let mut positions = Vec::new();

    let mut current_guard = Some(guard);
    while current_guard.is_some() {
        positions.push(current_guard.unwrap().pos);
        current_guard = get_next_position(&current_guard.unwrap(), map);
    }

    positions
}

fn parse_input(input: &str) -> MyResult<(Map, Guard)> {
    let mut obstacle_positions = HashSet::new();
    let mut guard_pos: Option<Point> = None;
    let mut guard_dir: Option<Direction> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    obstacle_positions.insert(Point { x, y });
                }
                '>' | '<' | '^' | 'V' => guard_pos = Some(Point { x, y }),
                _ => {}
            }

            match char {
                '^' => guard_dir = Some(Direction::Up),
                'V' => guard_dir = Some(Direction::Down),
                '<' => guard_dir = Some(Direction::Left),
                '>' => guard_dir = Some(Direction::Right),
                _ => {}
            }
        }
    }

    let dim = Dimensions {
        height: input.lines().count(),
        width: input.lines().map(|l| l.len()).max().unwrap(),
    };
    let map = Map {
        obstacles: obstacle_positions,
        dim,
    };
    let guard = Guard {
        pos: guard_pos.unwrap(),
        dir: guard_dir.unwrap(),
    };

    Ok((map, guard))
}
