use std::{collections::HashSet, iter::Filter};

use itertools::Itertools;

use crate::{MyResult, print_challenge_header};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct MappingRange {
    start: u64,
    destination: u64,
    length: u64,
}

struct PuzzleInput {
    seeds: Vec<u64>,
    mappings: Vec<Vec<MappingRange>>
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(5);

    println!("The lowest location number is: {}", solve_part_one(INPUT));
    println!("The actual lowest location number is: {}", solve_part_two(INPUT));

    Ok(())
}

pub fn solve_part_two(input:&str) -> u64 {
    let puzzle = parse_input(input);

    assert!(puzzle.seeds.len() % 2 == 0);

    let seeds = puzzle.seeds
        .iter()
        .copied()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, value)| value)
        .collect_vec();

    let lengths = puzzle.seeds
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, value)| value)
        .collect_vec();

    let mut lowest_location = u64::MAX;
    let mut solved_seeds=HashSet::new();
    for (start_seed, length) in seeds.into_iter().zip(lengths) {
        let seed_range = (start_seed..(start_seed+length))
            .into_iter().filter(|s| !solved_seeds.contains(s))
            .collect_vec(); 

        let location = determine_lowest_location(&seed_range, &puzzle.mappings);
        solved_seeds.extend(seed_range);

        if location < lowest_location {
            lowest_location = location;
        }
        
    }

    lowest_location

}

fn solve_part_one(input: &str) -> u64 {
    let puzzle_input = parse_input(input);
    determine_lowest_location(&puzzle_input.seeds, &puzzle_input.mappings)
}

fn determine_lowest_location(seeds: &[u64], mappings: &[Vec<MappingRange>])-> u64 {

    let mut lowest_location  = u64::MAX;

    for seed in seeds.iter().copied() {
        let mut source = seed;
        for mapping in mappings {
            source = get_mapped_destination(source, mapping);
        }

        if source < lowest_location {
            lowest_location = source;
        }
    }

    lowest_location
}

fn get_mapped_destination(source: u64, mapping: &[MappingRange]) -> u64 {
    for map in mapping {
        if source >= map.start && source < (map.start + map.length) {
            let offset = source - map.start;
            return map.destination + offset;
        } 
    }


    source
}

fn parse_input(input: &str) -> PuzzleInput {
    PuzzleInput { 
        seeds: parse_seeds(input),
         mappings: parse_mappings(input)
        }
}

fn parse_seeds(input: &str)-> Vec<u64> {
    let seeds = input
        .lines()
        .into_iter()
        .take(1)
        .map(|line| 
            line
                .split(' ')
                .skip(1)
                .map(|str| str.parse::<u64>().unwrap())
            )
        .flatten()
        .collect::<Vec<_>>();
        
    seeds
}

fn parse_mappings(input: &str) -> Vec<Vec<MappingRange>> {
    let mappings = input
        .split("\r\n\r\n")
        .skip(1)
        .map(parse_mapping_paragraph)
        .collect_vec();

    assert_eq!(mappings.len(), 7);

    mappings
}


fn parse_mapping_paragraph(paragraph: &str) -> Vec<MappingRange> {
    let ranges = paragraph
        .lines()
        .skip(1)
        .map(|line|
            line
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec()
        )
        .inspect(|numbers| assert_eq!(numbers.len(), 3) )
        .map(|numbers| MappingRange { destination: numbers[0], start: numbers[1], length: numbers[2] })
        .collect::<Vec<_>>();

    ranges
}


#[cfg(test)]
mod tests {
    use crate::year_2023::day_05::{INPUT, solve_part_two};

    use super::solve_part_one;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn part_one_example_input_solved_correctly() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert_eq!(result, 35);
    }

    #[test]
    fn part_one_real_input_solved_correctly() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 309796150);
    }

    #[test]
    fn part_two_example_input_solved_correctly() {
        let result = solve_part_two(EXAMPLE_INPUT);
        assert_eq!(result, 46);
    }

}