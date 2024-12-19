use anyhow::{bail, Result};
use itertools::Itertools;
use regex::Regex;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

struct PuzzleInput {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn get_valid_designs(puzzle_input: &PuzzleInput) -> Result<Vec<String>> {
    let pattern = puzzle_input.patterns.join("|");
    let reg = Regex::new(format!("^({})+$", pattern).as_str())?;

    let designs = puzzle_input
        .designs
        .iter()
        .filter(|design| reg.is_match(design))
        .cloned()
        .collect_vec();

    Ok(designs)
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
