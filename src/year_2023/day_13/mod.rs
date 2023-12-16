use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challenge_header(13);

    println!("The number after summarizing is {}", solve_part_one(INPUT));

    Ok(())
}

struct ParsedPattern {
    rows: Vec<u64>,
    columns: Vec<u64>,
}

fn solve_part_one(input: &str) -> usize {
    let patterns = parse_input(input);

    let sum = patterns
        .iter()
        .map(|p| calculate_number_for_pattern(p))
        .sum();

    sum
}

fn parse_input(input: &str) -> Vec<ParsedPattern> {
    let mut patterns = Vec::new();
    for pattern in input.split("\r\n\r\n") {
        patterns.push(parse_pattern(pattern));
    }

    patterns
}

fn parse_pattern(pattern: &str) -> ParsedPattern {
    ParsedPattern {
        rows: parse_pattern_by_row(pattern),
        columns: parse_pattern_by_column(pattern),
    }
}

fn parse_pattern_by_row(pattern: &str) -> Vec<u64> {
    pattern.lines().map(|l| calculate_hash(&l)).collect_vec()
}

fn parse_pattern_by_column(pattern: &str) -> Vec<u64> {
    let lines = pattern.lines().collect_vec();
    let mut columns = Vec::new();

    for (idx, _) in lines.first().unwrap().chars().enumerate() {
        let column = lines
            .iter()
            .map(|l| l.chars().skip(idx).take(1).last().unwrap())
            .collect::<String>();

        columns.push(calculate_hash(&column));
    }

    columns
}

fn calculate_number_for_pattern(pattern: &ParsedPattern) -> usize {
    if let Some(rows) = find_point_of_reflection(&pattern.rows) {
        return rows * 100;
    }

    if let Some(columns) = find_point_of_reflection(&pattern.columns) {
        return columns;
    }

    panic!();
}

fn find_point_of_reflection(pattern: &[u64]) -> Option<usize> {
    for (idx, window) in pattern.windows(2).enumerate() {
        let (left, right) = (window[0], window[1]);

        if left != right {
            continue;
        }

        if validate_reflection(pattern, idx) {
            return Some(idx + 1);
        }
    }

    None
}

fn validate_reflection(pattern: &[u64], index: usize) -> bool {
    let left_side = pattern.iter().take(index + 1).rev();
    let right_side = pattern.iter().skip(index + 1);

    left_side.zip(right_side).all(|(left, right)| left == right)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day_13::INPUT;

    use super::solve_part_one;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn solve_part_one_example_correctly() {
        let result = solve_part_one(EXAMPLE);
        assert_eq!(result, 405);
    }

    #[test]
    fn solve_part_one_input_correctly() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 33780);
    }
}
