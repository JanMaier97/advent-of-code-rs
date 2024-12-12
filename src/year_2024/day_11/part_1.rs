use macros::aoc_solver;

use crate::{year_2024::day_11::parse_input, MyResult};

use super::blink;

#[aoc_solver(2024, 11, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let values = parse_input(input)?;
    let count = blink(25, values).len();
    Ok(count.try_into()?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 55312);
    }
}
