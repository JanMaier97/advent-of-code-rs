use itertools::Itertools;
use macros::aoc_solver;

use super::parse_input;

use anyhow::{bail, Result};

#[aoc_solver(2024, 17, 2, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let rom = parse_input(input)?;

    let start_values = find_first_set_of_numbers(*rom.program.last().unwrap() as u128);
    let candidates = iterate_set_of_numbers(&start_values, &rom.program[..&rom.program.len() - 1]);
    let res = filter_candiate(&candidates, &rom.program)?;

    Ok(res.to_string())
}

fn filter_candiate(candidates: &[u128], output: &[u8]) -> Result<u128> {
    let output = output.iter().cloned().map(|v| v as u128).collect_vec();
    for start_value in candidates.iter().sorted() {
        if does_generate_output(*start_value, &output) {
            return Ok(*start_value);
        }
    }

    bail!("no result found");
}

fn iterate_set_of_numbers(start_values: &[u128], remainding_outputs: &[u8]) -> Vec<u128> {
    let target_outputs = remainding_outputs.iter().cloned().rev().collect_vec();
    let mut potential_values = start_values.to_vec();
    let mut next_values = Vec::new();

    for &target_output in target_outputs.iter() {
        let target_output = target_output as u128;

        for value in potential_values.iter() {
            let shifted_a = value << 3;
            let next_step_values = vec![
                shifted_a,
                shifted_a + 1,
                shifted_a + 2,
                shifted_a + 3,
                shifted_a + 4,
                shifted_a + 5,
                shifted_a + 6,
                shifted_a + 7,
            ];
            for v in next_step_values {
                let actual_output = compute_single_output_value(v);
                if actual_output == target_output {
                    next_values.push(v);
                }
            }
        }

        potential_values.clone_from(&next_values);
        next_values.clear();
    }

    potential_values
}

fn find_first_set_of_numbers(last_value: u128) -> Vec<u128> {
    let mut values = Vec::new();
    let start = 0b0000000;
    let end = 0b00000011_11111111;

    for a in start..=end {
        let value = compute_single_output_value(a);
        if value == last_value {
            values.push(a);
        }
    }

    values
}

fn compute_single_output_value(a: u128) -> u128 {
    ((((a % 8) ^ 1) ^ (a / 2_u128.pow(((a % 8_u128) ^ 1) as u32))) ^ 4) % 8
}

fn does_generate_output(a: u128, values: &[u128]) -> bool {
    let mut current_a = a;
    for value in values {
        let output = compute_single_output_value(current_a);
        if output != *value {
            return false;
        }

        current_a /= 8;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::does_generate_output;

    #[test]
    fn check_values() {
        let values: Vec<u128> = vec![1, 3, 7, 4, 6, 4, 2, 3, 5];
        let res = does_generate_output(30553366, &values);
        assert!(res)
    }

    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("input.txt")).unwrap();
        assert_eq!(result, "202367025818154");
    }
}
