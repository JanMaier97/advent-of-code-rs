use anyhow::Result;
use macros::aoc_solver;

use super::{get_valid_designs, parse_input};

#[aoc_solver(2024, 19, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let input = parse_input(input)?;
    let desings = get_valid_designs(&input)?;
    Ok(desings.len().to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "6");
    }
}
