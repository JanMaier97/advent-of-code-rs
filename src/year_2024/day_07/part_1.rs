use macros::aoc_solver;

use anyhow::Result;

use super::{compute_solution, Operator};

#[aoc_solver(2024, 7, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    compute_solution(input, &[Operator::Add, Operator::Multiply])
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "3749");
    }
}
