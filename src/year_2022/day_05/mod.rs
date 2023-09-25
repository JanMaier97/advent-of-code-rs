use itertools::Itertools;
use regex::Regex;

use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challange_header(5);

    println!(
        "1) The crates at the top of each stacks are {}",
        get_solution_for_first_part(INPUT)?
    );
    println!(
        "2) The crates at the top of each stacks are {}",
        get_solution_for_second_part(INPUT)?
    );

    Ok(())
}

fn get_solution_for_first_part(input: &str) -> MyResult<String> {
    let (mut stacks, moves) = parse_input_file(input)?;

    apply_moves_to_stacks_individually(&mut stacks, &moves);

    Ok(get_crate_labels_from_stacks(&stacks))
}

fn get_solution_for_second_part(input_file: &str) -> MyResult<String> {
    let (mut stacks, moves) = parse_input_file(input_file)?;

    apply_moves_to_stacks_in_bulck(&mut stacks, &moves);

    Ok(get_crate_labels_from_stacks(&stacks))
}

fn get_crate_labels_from_stacks(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .map(|s| s.last().unwrap_or(&' '))
        .collect::<String>()
}

fn parse_input_file(input: &str) -> MyResult<(Vec<Vec<char>>, Vec<(u32, usize, usize)>)> {
    let mut reversed_stacks = Vec::new();
    let mut moves = Vec::new();
    for line in input.lines() {
        if !line.starts_with("move") {
            let indexed_crates = parse_stacks(&line)?;
            for (index, crate_label) in indexed_crates {
                while reversed_stacks.len() < index + 1 {
                    reversed_stacks.push(Vec::new());
                }

                reversed_stacks[index].push(crate_label);
            }
        } else {
            moves.push(parse_moves(&line)?);
        }
    }

    let stacks = reversed_stacks
        .into_iter()
        .map(|s| s.into_iter().rev().collect_vec())
        .collect_vec();

    Ok((stacks, moves))
}

fn apply_moves_to_stacks_individually(
    crate_stacks: &mut [Vec<char>],
    actions: &[(u32, usize, usize)],
) {
    for &(count, source_stack, target_stack) in actions {
        for _ in 0..count {
            if let Some(popped_crate) = crate_stacks[source_stack].pop() {
                crate_stacks[target_stack].push(popped_crate);
            }
        }
    }
}

fn apply_moves_to_stacks_in_bulck(crate_stacks: &mut [Vec<char>], actions: &[(u32, usize, usize)]) {
    for &(count, source_stack, target_stack) in actions {
        let start_index = crate_stacks[source_stack].len() - count as usize;
        let crates_to_move = crate_stacks[source_stack].split_off(start_index);
        crate_stacks[target_stack].extend(crates_to_move);
    }
}

fn parse_stacks(line: &str) -> MyResult<Vec<(usize, char)>> {
    let mut result = Vec::new();
    for (index, chunk) in line.chars().chunks(4).into_iter().enumerate() {
        let l = chunk.collect::<String>();

        if l.len() < 3 {
            return Err("Malformed stacks".into());
        }

        if l.chars().any(|c| c.is_ascii_digit()) {
            break;
        }

        if l.chars().all(|c| c == ' ') {
            continue;
        }

        let crate_label = l.chars().nth(1).unwrap();
        if !crate_label.is_ascii_uppercase() {
            return Err("Invalid crate label".into());
        }

        result.push((index, crate_label));
    }

    Ok(result)
}

fn parse_moves(line: &str) -> MyResult<(u32, usize, usize)> {
    // for line in lines {}
    let reg = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let cap = reg.captures(line).unwrap();
    let (count, source_stack, target_stack) = (
        cap[1].parse()?,
        cap[2].parse::<i32>()? - 1,
        cap[3].parse::<i32>()? - 1,
    );

    if count < 1 || source_stack < 0 || target_stack < 0 {
        return Err("Invalid index for move".into());
    }

    Ok((count, source_stack as usize, target_stack as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn get_solution_for_first_part_example() {
        let result = get_solution_for_first_part(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "CMZ");
    }

    #[test]
    fn get_solution_for_first_part_real() {
        let result = get_solution_for_first_part(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "RFFFWBPNS");
    }

    #[test]
    fn get_solution_for_second_part_example() {
        let result = get_solution_for_second_part(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "MCD");
    }
}
