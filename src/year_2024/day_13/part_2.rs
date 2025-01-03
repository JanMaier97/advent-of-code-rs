use macros::aoc_solver;

use anyhow::Result;

use super::solve_for_input;

#[aoc_solver(2024, 13, 2, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    solve_for_input(input, 10_000_000_000_000)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "875318608908");
    }
}
