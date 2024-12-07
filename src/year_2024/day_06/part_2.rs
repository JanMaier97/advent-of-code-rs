use std::collections::HashSet;

use crate::{MyResult, SolverMetadata, SOLVERS};
use linkme::distributed_slice;

use super::{determine_guard_path, parse_input, Direction, Guard, Map, Point};

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 6,
    part: 2,
    func: solve,
    input: super::INPUT,
};

fn solve(input: &str) -> MyResult<u32> {
    let (map, guard) = parse_input(input)?;
    let positions = generate_loop_possitions(guard, &map);

    Ok(positions.len() as u32)
}

fn generate_loop_possitions(guard: Guard, map: &Map) -> HashSet<Point> {
    let path = determine_guard_path(guard, map);
    let mut possible_positions = HashSet::new();

    for point in path
        .into_iter()
        .filter(|pos| *pos != guard.pos && !map.obstacles.contains(pos))
    {
        let mut new_map = map.clone();
        new_map.obstacles.insert(point);

        if check_is_loop(guard, &new_map) {
            possible_positions.insert(point);
        }
    }

    possible_positions
}

fn check_is_loop(guard: Guard, map: &Map) -> bool {
    let mut bi_grams: HashSet<(Guard, Guard)> = HashSet::new();

    let mut current_guard = guard;
    loop {
        let Some(next_guard) = super::get_next_position(&current_guard, map) else {
            return false;
        };

        let bi_gram = (current_guard, next_guard);
        if bi_grams.contains(&bi_gram) {
            return true;
        }

        bi_grams.insert(bi_gram);

        current_guard = next_guard;
    }
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
    use crate::year_2024::day_06::parse_input;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn solve_exampe() {
        let result = super::solve(EXAMPLE).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn example_map_is_not_a_loop() {
        let (map, guard) = parse_input(EXAMPLE).unwrap();
        let is_loop = super::check_is_loop(guard, &map);
        assert!(!is_loop)
    }

    #[test]
    fn simple_loop_map_is_loop() {
        let (map, guard) = parse_input(include_str!("simple_loop.txt")).unwrap();
        let is_loop = super::check_is_loop(guard, &map);
        assert!(is_loop)
    }

    #[test]
    fn thin_loop_map_is_loop() {
        let (map, guard) = parse_input(include_str!("long_loop.txt")).unwrap();
        let is_loop = super::check_is_loop(guard, &map);
        assert!(is_loop)
    }
}
