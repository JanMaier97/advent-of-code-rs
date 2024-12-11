use macros::aoc_solver;

use crate::MyResult;

use super::{get_next_positions, parse_input, Map, Position};

#[aoc_solver(2024, 10, 2, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let puzzle_input = parse_input(input)?;
    let sum = puzzle_input
        .start_positions
        .iter()
        .map(|pos| compute_rating(*pos, &puzzle_input.map))
        .sum();

    Ok(sum)
}

fn compute_rating(start_position: Position, map: &Map) -> u64 {
    depth_first_search(start_position, map)
}

fn depth_first_search(current_position: Position, map: &Map) -> u64 {
    if map.grid[current_position.y][current_position.x] == 9 {
        return 1;
    }

    let next_positions = get_next_positions(current_position, map);
    next_positions
        .iter()
        .map(|pos| depth_first_search(*pos, map))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 81);
    }
}
