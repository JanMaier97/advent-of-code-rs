use macros::aoc_solver;

use anyhow::Result;

use super::{all_levels_safe, parse_and_count};

#[aoc_solver(2024, 2, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    parse_and_count(input, all_levels_safe)
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn solve_example() {
        let result = super::solve(EXAMPLE).unwrap();
        assert_eq!(result, "2");
    }
}
