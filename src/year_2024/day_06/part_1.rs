use std::collections::HashSet;

use macros::aoc_solver;

use anyhow::Result;

use super::{determine_guard_path, parse_input, Direction, Guard, Map, Point};

#[aoc_solver(2024, 6, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let (map, guard) = parse_input(input)?;
    let positions = compute_guard_positions(guard, &map);

    let res = u64::try_from(positions.len())?;
    Ok(res.to_string())
}

fn compute_guard_positions(guard: Guard, map: &Map) -> HashSet<Point> {
    determine_guard_path(guard, map)
        .into_iter()
        .collect::<HashSet<_>>()
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
        assert_eq!(result, "41");
    }
}
