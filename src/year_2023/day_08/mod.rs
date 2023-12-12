use std::{collections::HashMap, f32::consts::E};

use itertools::Itertools;
use rayon::collections::hash_map;
use regex::Regex;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

enum Direction {
    Left,
    Right,
}

struct Edge {
    left: String,
    right: String,
}

struct PuzzleInput {
    directions: Vec<Direction>,
    network: HashMap<String, Edge>,
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(7);

    println!("The total number of steps is {}", solve_part_one(INPUT));
    println!("The total winnings are {}", solve_part_two(INPUT));

    Ok(())
}


fn solve_part_one(input: &str) -> u32 {
    let puzzle_input = parse_input(input);

    let mut step_count = 0;
    let mut current_node = "AAA";
    loop {
        let edge = puzzle_input.network.get(current_node).unwrap();
        
        let direction = &puzzle_input.directions[step_count%puzzle_input.directions.len()];
        current_node= match direction {
            Direction::Left => &edge.left,
            Direction::Right => &edge.right,
        };

        step_count += 1;

        if current_node == "ZZZ" {
            break;
        }
    }

    step_count as u32
}

fn solve_part_two(input: &str) -> u32 {
    let puzzle_input = parse_input(input);

    let mut step_count = 0;
    let mut start_nodes = puzzle_input.network
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect_vec();

    loop {
        let direction = &puzzle_input.directions[step_count%puzzle_input.directions.len()];
        start_nodes = start_nodes.iter()
            .map(|&node| puzzle_input.network.get(node).unwrap())
            .map(|edge| match direction {
            Direction::Left => &edge.left,
            Direction::Right =>&edge.right,
        }).collect::<Vec<_>>();

        step_count += 1;

        if start_nodes.iter().all(|n| n.ends_with("Z")){
            break;
        }
    }

    step_count as u32

}

fn parse_input(input: &str) -> PuzzleInput {
    PuzzleInput { directions: parse_directions(input), network: parse_network(input) }
}

fn parse_directions(input: &str) -> Vec<Direction> {
    let line = input
        .lines()
        .take(1)
        .collect::<String>();

    line.chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!()
        })
        .collect_vec()
}

fn parse_network(input: &str) -> HashMap<String, Edge> {
    let reg = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

    input.lines()
        .skip(2)
        .map(|l| reg.captures(l).unwrap())
        .map(|capture| (capture[1].to_string(), Edge { left: capture[2].to_string(), right: capture[3].to_string() }))
        .collect::<HashMap<_,_>>()
}


#[cfg(test)]
mod tests {
    use crate::year_2023::day_08::{solve_part_one, solve_part_two, INPUT};

    const EXAMPLE1_INPUT: &str = include_str!("example1.txt");
    const EXAMPLE2_INPUT: &str = include_str!("example2.txt");
    const EXAMPLE3_INPUT: &str = include_str!("example3.txt");

    #[test]
    fn part_one_example1_input_correct() {
        let result = solve_part_one(EXAMPLE1_INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_one_example2_input_correct() {
        let result = solve_part_one(EXAMPLE2_INPUT);
        assert_eq!(result, 6);
    }


    #[test]
    fn part_one_real_input_correct() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 16579);
    }

    #[test]
    fn part_two_example3_input_correct() {
        let result = solve_part_two(EXAMPLE3_INPUT);
        assert_eq!(result, 6);
    }


    #[test]
    fn part_two_real_input_correct() {
        let result = solve_part_two(INPUT);
        assert_eq!(result, 0);
    }
}