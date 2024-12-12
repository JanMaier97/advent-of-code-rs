use itertools::Itertools;

use crate::MyResult;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> MyResult<Vec<u64>> {
    let res = input
        .split(' ')
        .map(|part| part.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(res)
}

fn blink(times: u64, start: Vec<u64>) -> Vec<u64> {
    let mut result = start;
    for _ in 0..times {
        result = result
            .iter()
            .flat_map(|value| apply_rules(*value))
            .collect_vec();
    }

    result
}

fn apply_rules(value: u64) -> Vec<u64> {
    if value == 0 {
        return vec![1];
    }

    let str_value = value.to_string();
    if str_value.len() % 2 == 0 {
        let middle = str_value.len() / 2;
        return vec![
            str_value[..middle].parse::<u64>().unwrap(),
            str_value[middle..].parse::<u64>().unwrap(),
        ];
    }

    vec![value * 2024]
}
