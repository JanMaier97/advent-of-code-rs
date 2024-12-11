use macros::aoc_solver;

use crate::MyResult;

use super::parse_input;

#[aoc_solver(2024, 1, 1, super::INPUT)]
pub fn solve(input: &str) -> MyResult<u64> {
    let mut locations = parse_input(input)?;

    locations.list1.sort();
    locations.list2.sort();

    let result: u32 = locations
        .list1
        .into_iter()
        .zip(locations.list2)
        .map(|(loc1, loc2)| loc1.abs_diff(loc2))
        .sum();

    Ok(result.into())
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_01::part_1::solve;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn solve_example() {
        let result = solve(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 11);
    }
}
