use macros::aoc_solver;

use anyhow::Result;

use super::solve_for_input;

#[aoc_solver(2024, 13, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    solve_for_input(input, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "480");
    }
}
