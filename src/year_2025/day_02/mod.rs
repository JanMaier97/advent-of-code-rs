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

#[aoc_solver(2025, 2, 2, INPUT)]
fn solve_part_2(input: &str) -> Result<String> {
    let mut sum = 0;

    for value in input.trim().split(',').flat_map(|r| parse_range(r)) {
        let is_invalid = contains_repated_digits(value);
        if is_invalid {
            sum += value;
        }
    }

    return Ok(sum.to_string());
}

fn contains_repated_digits(value: u64) -> bool {
    if value < 10 {
        return false;
    }

    let value = value.to_string().chars().collect::<Vec<_>>();

    for chunk_size in (1..=(value.len() / 2)).rev() {
        let chunks = value.chunks(chunk_size).collect::<Vec<_>>();
        let first = chunks.first().unwrap();
        let all_chunks_equal = chunks.iter().all(|c| c == first);

        if all_chunks_equal {
            return true;
        }
    }

    return false;
}

fn parse_range(range_str: &str) -> RangeInclusive<u64> {
    let mut nums = range_str.split('-');

    let lower = nums.next().unwrap().parse::<u64>().unwrap();
    let upper = nums.next().unwrap().parse::<u64>().unwrap();

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
    fn solve_example_part_2() {
        let result = super::solve_part_2(include_str!("example.txt")).unwrap();
        assert_eq!(result, "4174379265");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve(super::INPUT).unwrap();
        assert_eq!(result, "35367539282");
    }

    #[test]
    fn solve_part_2() {
        let result = super::solve_part_2(super::INPUT).unwrap();
        assert_eq!(result, "4174379265");
    }
}
