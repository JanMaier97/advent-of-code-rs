use std::collections::VecDeque;

use itertools::Itertools;
use num::{BigUint, Zero};
use regex::Regex;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum OperationValue {
    Old,
    Number(BigUint),
}

#[derive(Debug, Copy, Clone)]
enum OperationType {
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
struct MonkeyTest {
    divider: u32,
    true_target: u32,
    false_target: u32,
}

#[derive(Debug)]
struct MonkeyConfig {
    id: u32,
    items: VecDeque<BigUint>,
    operation_value: OperationValue,
    operation_type: OperationType,
    test: MonkeyTest,
    inpect_count: u32,
}

impl MonkeyConfig {
    fn parse_config(lines: &[String]) -> MyResult<Self> {
        if lines.len() != 6 {
            return Err("Invalid line count".into());
        }

        let header_regex = Regex::new(r"^Monkey (\d+):$")?;
        let items_regex = Regex::new(r"^Starting items: ((\d+(, ){0,1})+)$")?;
        let operation_regex = Regex::new(r"^Operation: new = old ([+*-]) (old|(\d)+)$")?;
        let test_regex = Regex::new(r"^Test: divisible by (\d+)$")?;
        let test_true_regex = Regex::new(r"^If true: throw to monkey (\d+)$")?;
        let test_false_regex = Regex::new(r"^If false: throw to monkey (\d+)$")?;

        let Some(header_match) = header_regex.captures(lines[0].trim()) else {
            return Err("Invalid header".into());
        };

        let id = header_match[1].parse::<u32>()?;

        let Some(items_capture) = items_regex.captures(lines[1].trim()) else {
            return Err("Invalid starting items".into());
        };

        let items = items_capture[1]
            .split(',')
            .map(|i| i.trim().parse::<BigUint>())
            .try_collect()?;

        let Some(operation_capture) = operation_regex.captures(lines[2].trim()) else {
            return Err("Invalid operation".into());
        };

        let operation_type = match &operation_capture[1] {
            "*" => OperationType::Mul,
            "+" => OperationType::Add,
            "-" => OperationType::Sub,
            _ => return Err("Invalid operation".into()),
        };

        let operation_value = match &operation_capture[2] {
            "old" => OperationValue::Old,
            _ => OperationValue::Number(operation_capture[2].parse::<BigUint>()?),
        };

        let Some(test_capture) = test_regex.captures(lines[3].trim()) else {
            return Err("Invalid test format".into());
        };

        let divider = test_capture[1].parse::<u32>()?;

        let Some(test_true_capture) = test_true_regex.captures(lines[4].trim()) else {
            return Err("Invalid test true format".into());
        };

        let true_target = test_true_capture[1].parse::<u32>()?;

        let Some(test_false_capture) = test_false_regex.captures(lines[5].trim()) else {
            return Err("Invalid test false format".into());
        };

        let false_target = test_false_capture[1].parse::<u32>()?;

        Ok(MonkeyConfig {
            id,
            items,
            operation_type,
            operation_value,
            inpect_count: 0,
            test: MonkeyTest {
                divider,
                true_target,
                false_target,
            },
        })
    }
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(11);

    println!(
        "1) The level of monkey business is {}",
        solve_part_one(INPUT)?
    );
    println!(
        "2) the level of monkey business is {}",
        solve_part_two(INPUT)?
    );

    Ok(())
}

fn solve_part_one(input: &str) -> MyResult<u32> {
    execute_monkey_game(input, 20, 3)
}

fn solve_part_two(file: &str) -> MyResult<u32> {
    execute_monkey_game(file, 10_000, 1)
}

fn execute_monkey_game(file: &str, rounds: usize, worry_div: u32) -> MyResult<u32> {
    let mut monkeys = parse_input(file)?;

    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            // println!("Monkey: {}:", monkeys[idx].id);
            loop {
                let Some(item) = &monkeys[idx].items.pop_front() else {
                    break;
                };

                monkeys[idx].inpect_count += 1;

                let operation_val = match &monkeys[idx].operation_value {
                    OperationValue::Old => item,
                    OperationValue::Number(value) => value,
                };

                let inspected_item = match &monkeys[idx].operation_type {
                    OperationType::Add => item + operation_val,
                    OperationType::Mul => item * operation_val,
                    OperationType::Sub => item - operation_val,
                };

                let item_before_test = inspected_item / worry_div;

                let target_monkey_id =
                    if &item_before_test % monkeys[idx].test.divider == Zero::zero() {
                        monkeys[idx].test.true_target
                    } else {
                        monkeys[idx].test.false_target
                    };

                let target_monkey = monkeys.iter_mut().find(|m| m.id == target_monkey_id);
                target_monkey.unwrap().items.push_back(item_before_test);

                // println!("  Inspecting {}", item);
                // println!(
                //     "    Operation {:?} by {} to {}",
                //     monkeys[idx].operation_type, operation_val, inspected_item
                // );
                // println!("    Dividied by 3 to {} ", item_before_test);
                // println!(
                //     "    Item {} thrown to {} ",
                //     item_before_test, target_monkey_id
                // );
            }
        }
    }

    // println!("Monkey 1: {:?}", monkeys[0].items);
    // println!("Monkey 2: {:?}", monkeys[1].items);
    // println!("Monkey 3: {:?}", monkeys[2].items);
    // println!("Monkey 4: {:?}", monkeys[3].items);

    let inspection_counts = monkeys
        .iter()
        .map(|m| m.inpect_count)
        .sorted()
        .rev()
        .take(2)
        .fold(1_u32, |acc, val| acc * val);

    Ok(inspection_counts)
}

fn parse_input(input: &str) -> MyResult<Vec<MonkeyConfig>> {
    let mut configs = Vec::new();

    let mut line_buffer = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            configs.push(MonkeyConfig::parse_config(&line_buffer)?);
            line_buffer.clear();
        } else {
            line_buffer.push(line.to_string());
        }
    }

    if !line_buffer.is_empty() {
        configs.push(MonkeyConfig::parse_config(&line_buffer)?);
    }

    // TODO: add checks to ensure that referenced ids actually exist

    Ok(configs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn solve_part_one_example() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10605);
    }

    #[test]
    fn solve_part_one_real() {
        let result = solve_part_one(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 182293);
    }

    // text executions takes too long
    //    #[test]
    //    fn solve_part_two_example() {
    //        let result = solve_part_two(EXAMPLE_INPUT);
    //        assert!(result.is_ok());
    //        assert_eq!(result.unwrap(), 2713310158);
    //    }
}
