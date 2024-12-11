use std::collections::HashSet;

use macros::aoc_solver;

use crate::MyResult;

use super::{get_next_positions, parse_input, Map, Position};

#[aoc_solver(2024, 10, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let puzzle_input = parse_input(input)?;
    let total_score = puzzle_input
        .start_positions
        .iter()
        .map(|pos| score_trail(*pos, &puzzle_input.map))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(total_score)
}

fn score_trail(start_position: Position, map: &Map) -> MyResult<u64> {
    score_recursively(HashSet::from([start_position]), map)
}

fn score_recursively(positions: HashSet<Position>, map: &Map) -> MyResult<u64> {
    if positions.is_empty() {
        return Ok(0);
    }

    let next_positions = positions
        .iter()
        .flat_map(|pos| get_next_positions(*pos, map))
        .collect::<HashSet<_>>();
    let next_score = score_recursively(next_positions, map)?;
    let current_score: u64 = positions
        .iter()
        .map(|pos| map.grid[pos.y][pos.x])
        .filter(|value| *value == 9)
        .count()
        .try_into()?;

    Ok(current_score + next_score)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 36);
    }
}
