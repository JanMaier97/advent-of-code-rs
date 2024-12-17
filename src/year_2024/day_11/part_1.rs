use macros::aoc_solver;

use crate::year_2024::day_11::parse_input;

use super::blink;

use anyhow::Result;

#[aoc_solver(2024, 11, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let values = parse_input(input)?;
    let count = blink(25, values).len();
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "55312");
    }
}
