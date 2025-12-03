use std::ops::RangeInclusive;

use macros::aoc_solver;

use anyhow::Result;

static INPUT: &str = include_str!("input.txt");

#[aoc_solver(2025, 2, 1, INPUT)]
pub fn solve(input: &str) -> Result<String> {
    let sum_of_invalid_ids: u64 = input
        .trim()
        .split(',')
        .flat_map(|r| parse_range(r).filter(|id| !is_valid_id(*id)))
        .sum();

    return Ok(sum_of_invalid_ids.to_string());
}

fn parse_range(range_str: &str) -> RangeInclusive<u64> {
    let mut nums = range_str.split('-');

    let lower = dbg!(nums.next().unwrap()).parse::<u64>().unwrap();
    let upper = dbg!(nums.next().unwrap()).parse::<u64>().unwrap();

    return lower..=upper;
}

fn is_valid_id(value: u64) -> bool {
    let value = value.to_string();
    if value.len() % 2 != 0 {
        return true;
    }

    let (left, right) = value.split_at(value.len() / 2);
    return left != right;
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "1227775554");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve(super::INPUT).unwrap();
        assert_eq!(result, "35367539282");
    }
}
