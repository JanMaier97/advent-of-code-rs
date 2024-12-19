use std::collections::HashMap;

use anyhow::Result;
use macros::aoc_solver;

use super::{get_valid_designs, parse_input};

#[aoc_solver(2024, 19, 2, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let input = parse_input(input)?;
    let designs = get_valid_designs(&input)?;

    let mut solved = HashMap::new();
    let sum: u64 = designs
        .iter()
        .map(|d| get_permutation_count(d, &input.patterns, &mut solved))
        .sum();

    Ok(sum.to_string())
}

fn get_permutation_count<'a>(
    design: &'a str,
    patterns: &[String],
    solved: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(count) = solved.get(design) {
        return *count;
    };

    if design.is_empty() {
        return 1;
    }

    let mut sum = 0;
    for pattern in patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern))
    {
        let simplified_desing = &design[pattern.len()..];
        let count = get_permutation_count(simplified_desing, patterns, solved);
        solved.insert(simplified_desing, count);
        sum += count;
    }

    sum
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "16");
    }
}
