use std::{collections::HashSet, hash::Hash};

use crate::{MyResult, SolverMetadata, SOLVERS};
use linkme::distributed_slice;

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 6,
    part: 1,
    func: solve,
    input: super::INPUT,
};

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

struct Map {
    obstacles: HashSet<Point>,
    dim: Dimensions,
}

fn solve(input: &str) -> MyResult<u32> {
    let (map, guard) = parse_input(input)?;
    let positions = compute_guard_positions(guard, &map);

    Ok(positions.len() as u32)
}

fn compute_guard_positions(guard: Guard, map: &Map) -> HashSet<Point> {
    let mut positions = HashSet::new();

    let mut current_guard = Some(guard);
    while current_guard.is_some() {
        positions.insert(current_guard.unwrap().pos);
        current_guard = get_next_position(&current_guard.unwrap(), map);
    }

    positions
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

#[allow(unused)]
fn print_map(map: &Map, guard: Guard) {
    for y in 0..map.dim.height {
        let line = (0..map.dim.width)
            .map(|x| Point { y, x })
            .map(|p| point_to_char(p, map, &guard))
            .collect::<String>();
        println!("{}", line);
    }
    println!("\n");
}

fn point_to_char(point: Point, map: &Map, guard: &Guard) -> char {
    if point == guard.pos {
        return match guard.dir {
            Direction::Up => '^',
            Direction::Down => 'V',
            Direction::Left => '<',
            Direction::Right => '>',
        };
    }

    if map.obstacles.contains(&point) {
        return '#';
    }

    '.'
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve_exampe() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 41);
    }
}
