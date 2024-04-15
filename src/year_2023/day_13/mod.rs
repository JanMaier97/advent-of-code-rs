use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::pattern::Pattern;

use itertools::{Itertools, Position};

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challenge_header(13);

    println!("The number after summarizing is {}", solve_part_one(INPUT));
    println!("The number after summarizing is {}", solve_part_two(INPUT));

    Ok(())
}

struct ParsedPattern {
    rows: Vec<String>,
    columns: Vec<String>,
}

struct ReflectionPoint {
    index: usize,
    smudges: Vec<Position>
}

fn solve_part_one(input: &str) -> usize {
    let patterns = parse_input(input);

    let sum = patterns
        .iter()
        .map(|p| calculate_number_for_pattern(p))
        .sum();

    sum
}

fn solve_part_two(input: &str) -> usize {
    let patterns = parse_input(input);

    for pattern in patterns
        .iter()
        .skip(1) {
            find_possible_smudge_in_pattern(&pattern.rows);
            // fix_smudge_in_pattern(&pattern.rows);
    }

    return 0;
}

fn find_and_fix_smudge(pattern: &[String]) -> Option<usize> {
    let positions = find_possible_smudge_in_pattern(pattern);
    None
}

fn find_possible_smudge_in_pattern(pattern: &[String]) {
    let points = find_all_possible_points_of_reflection(pattern)
        .into_iter()
        .collect_vec();

    println!("found possible mirror points: {:?}", points);

    for index in points {
        let left = pattern.iter().take(index+1).rev();
        let right = pattern.iter().skip(index+1);

        for (offset, (left, right)) in left.zip(right).enumerate()  {
            if left == right {
                continue;
            }

            let Some(smudge_pos) = find_smudge(&left, &right) else {
                println!("no smudge found");
                return;
            };

            println!("found smudge for mirror point pattern");
            println!("start idx: {}, outer idx: {}, inner idx: {}", index, index-offset, smudge_pos);
            println!("{}\n{}", left, right);
        }
    }


}

fn find_smudge(a: &str, b: &str)-> Option<usize> {
    let mut current_index = None;
    for (index, (left, right)) in a.chars().zip(b.chars()).enumerate() {
        if left == right {
            continue;
        }

        if current_index != None {
            return None;
        }

        current_index = Some(index);
    }

    current_index
}

fn replace_smudge(mut input: &str, index: usize) {

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

fn parse_pattern_by_row(pattern: &str) -> Vec<String> {
    pattern.lines().map(|l| l.to_string()).collect_vec()
}

fn parse_pattern_by_column(pattern: &str) -> Vec<String> {
    let lines = pattern.lines().collect_vec();
    let mut columns = Vec::new();

    for (idx, _) in lines.first().unwrap().chars().enumerate() {
        let column = lines
            .iter()
            .map(|l| l.chars().skip(idx).take(1).last().unwrap())
            .collect::<String>();

        columns.push(column);
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

fn find_point_of_reflection(pattern: &[String], edit_distance: u32) -> Option<usize> {
    for idx in find_all_possible_points_of_reflection(pattern, edit_distance){
        if validate_reflection(pattern, idx) {
            return Some(idx + 1);
        }
    }

    None
}

fn find_all_possible_points_of_reflection(pattern: &[String], edit_distance: u32) -> ReflectionPoint {
    let mut indices = Vec::new();
    for (idx, window) in pattern.windows(2).enumerate() {
        let (left, right) = (&window[0], &window[1]);

        if left != right {
            continue;
        }

        indices.push(idx);
    }

    indices
}


fn validate_reflection(pattern: &[String], index: usize) -> bool {
    let left_side = pattern.iter().take(index + 1).rev();
    let right_side = pattern.iter().skip(index + 1);

    left_side.zip(right_side).all(|(left, right)| left == right)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn change_indices(a: &str, b: &str) -> Vec<usize> {
    a.chars()
        .zip(b.chars())
        .enumerate()
        .filter(|(idx, (a,b))| a != b )
        .map(|(idx, _)| idx)
        .collect_vec()

}

#[cfg(test)]
mod tests {
    use crate::year_2023::day_13::{INPUT, solve_part_two};

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

    #[test]
    fn solve_part_two_example_correctly() {
        let result = solve_part_two(EXAMPLE);
        assert_eq!(result, 400);
    }

}
