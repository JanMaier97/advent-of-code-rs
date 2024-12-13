use macros::aoc_solver;

use crate::MyResult;

use super::solve_for_input;

#[aoc_solver(2024, 13, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    solve_for_input(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 480);
    }
}
