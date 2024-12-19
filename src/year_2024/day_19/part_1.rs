use anyhow::{bail, Result};
use itertools::Itertools;
use macros::aoc_solver;
use regex::Regex;

struct PuzzleInput {
    patterns: Vec<String>,
    designs: Vec<String>,
}

#[aoc_solver(2024, 19, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let puzzle_input = parse_input(input)?;

    let pattern = puzzle_input.patterns.join("|");
    let reg = Regex::new(format!("^({})+$", pattern).as_str())?;

    let valid_designs = puzzle_input
        .designs
        .iter()
        .filter(|design| reg.is_match(design))
        .count();

    Ok(valid_designs.to_string())
}

fn parse_input(input: &str) -> Result<PuzzleInput> {
    let blocks = input.split("\r\n\r\n").collect_vec();

    if blocks.len() != 2 {
        bail!("Expected 2 blocks");
    }

    let res = PuzzleInput {
        patterns: parse_patterns(blocks[0]),
        designs: parse_designs(blocks[1]),
    };

    Ok(res)
}

fn parse_designs(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect_vec()
}

fn parse_patterns(line: &str) -> Vec<String> {
    line.split(", ").map(|p| p.to_string()).collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "6");
    }
}
