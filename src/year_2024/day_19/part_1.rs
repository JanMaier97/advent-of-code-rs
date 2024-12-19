use anyhow::{bail, Result};
use itertools::Itertools;
use macros::aoc_solver;
use regex::Regex;

#[derive(Debug, Clone,Copy, PartialEq)]
enum Color {
    Black,
    Blue,
    Green,
    Red,
    White,
}

impl Color {
    fn from_char(char: char) -> Self {
        match char {
            'w' => Color::White,
            'b' => Color::Black,
            'u' => Color::Blue,
            'r' => Color::Red,
            'g' => Color::Green,
            _ => panic!("invalid color"),
        }
    }
}

struct PuzzleInput {
    patterns: Vec<Vec<Color>>,
    designs: Vec<Vec<Color>>,
}

#[aoc_solver(2024, 19, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let puzzle_input = parse_input(input)?;

    // let pattern = puzzle_input.patterns.
    let reg = Regex::new("()")?;

    let valid_designs = puzzle_input.designs
        .iter()
        .filter(|design| is_design_valid(design, &puzzle_input.patterns))
        .count();

    Ok(valid_designs.to_string())
}

fn is_design_valid(design: &[Color], patterns: &[Vec<Color>]) -> bool {
    
    let 

    false
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

fn parse_designs(input: &str) -> Vec<Vec<Color>> {
    input
        .lines()
        .map(|l| l.chars().map(Color::from_char).collect_vec())
        .collect_vec()
}

fn parse_patterns(line: &str) -> Vec<Vec<Color>> {
    line.split(", ")
        .map(|p| p.chars().map(Color::from_char).collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "22");
    }
}
