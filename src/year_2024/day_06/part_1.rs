use std::collections::HashSet;

use crate::{MyResult, SolverMetadata, SOLVERS};
use linkme::distributed_slice;

use super::{determine_guard_path, parse_input, Direction, Guard, Map, Point};

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 6,
    part: 1,
    func: solve,
    input: super::INPUT,
};

fn solve(input: &str) -> MyResult<u32> {
    let (map, guard) = parse_input(input)?;
    let positions = compute_guard_positions(guard, &map);

    Ok(positions.len() as u32)
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
        assert_eq!(result, 41);
    }
}
