use std::collections::HashSet;

use macros::aoc_solver;

use crate::MyResult;

use super::{determine_guard_path, parse_input, Direction, Guard, Map, Point};

#[aoc_solver(2024, 6, 2, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let (map, guard) = parse_input(input)?;
    let positions = generate_loop_possitions(guard, &map);
    let result = u64::try_from(positions.len())?;
    Ok(result)
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
    let mut bi_grams: HashSet<Guard> = HashSet::new();

    let mut current_guard = guard;
    loop {
        let Some(next_guard) = super::get_next_position(&current_guard, map) else {
            return false;
        };

        if bi_grams.contains(&next_guard) {
            return true;
        }

        bi_grams.insert(next_guard);

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
