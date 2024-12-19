use anyhow::Result;
use macros::aoc_solver;

#[aoc_solver(2024, 19, 2, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "22");
    }
}
