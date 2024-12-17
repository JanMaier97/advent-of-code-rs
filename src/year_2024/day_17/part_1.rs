use anyhow::Result;
use macros::aoc_solver;

#[aoc_solver(2024, 17, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }
}
