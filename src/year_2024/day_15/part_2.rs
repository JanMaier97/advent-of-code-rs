use macros::aoc_solver;

use crate::MyResult;


#[aoc_solver(2024, 15, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 10092);
    }

    #[test]
    fn solve_small_example() {
        let result = super::solve(include_str!("small_example.txt")).unwrap();
        assert_eq!(result, 10092);
    }
}