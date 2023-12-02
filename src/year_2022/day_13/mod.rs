use std::cmp::Ordering;
use std::collections::VecDeque;

use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

use CompareResult::*;
use PacketData::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Clone)]
enum PacketData {
    List(Vec<PacketData>),
    Value(u8),
}

#[derive(Debug, PartialEq)]
enum CompareResult {
    Valid,
    Invalid,
    Equivalent,
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(13);

    println!("The sum of indices is {}", solve_part_one(INPUT)?);

    Ok(())
}

fn solve_part_one(input: &str) -> MyResult<u32> {
    let mut result = 0;
    let mut chunk_index = 1;
    for chunk in &input.lines().chunks(3) {
        let chunk: Vec<&str> = chunk.collect_vec();

        if chunk.len() < 2 {
            return Err("Invalid file format".into());
        }

        let left = &chunk[0];
        let right = &chunk[1];

        // println!("comparing {:?} with {:?}", left, right);
        if compare_packets(&parse_line(&left)?, &parse_line(&right)?) == Valid {
            println!("Chunk {} is valid", chunk_index);
            result += chunk_index;
        }

        chunk_index += 1;
    }

    Ok(result)
}

fn solve_part_two() -> MyResult<u32> {
    todo!()
}

fn parse_line(line: &str) -> MyResult<PacketData> {
    if line.is_empty() {
        return Err("Line is empty".into());
    }

    if !line.ends_with("]") {
        return Err("Line does not end with ]".into());
    }

    if !line.starts_with("[") {
        return Err("Line does not start with [".into());
    }

    let mut token_stack = VecDeque::new();
    let chars = line.chars().collect_vec();
    let mut idx = 0;

    loop {
        if idx >= chars.len() {
            break;
        }

        let token = chars[idx];

        if token.is_ascii_digit() {
            let mut digit = token.to_string();

            loop {
                if !chars[idx + 1].is_ascii_digit() {
                    break;
                }

                digit.push(chars[idx + 1]);
                idx += 1;
            }
            token_stack.push_back(digit);
        } else {
            token_stack.push_back(token.to_string());
        }

        idx += 1;
    }

    let packet = resolve_tokens(&mut token_stack)?;

    Ok(packet)
}

fn resolve_tokens(tokens: &mut VecDeque<String>) -> MyResult<PacketData> {
    let mut result_stack: Vec<PacketData> = Vec::new();

    loop {
        let Some(token) = tokens.pop_front() else {
            return Ok(result_stack.first().unwrap().clone());
        };

        match token.as_str() {
            "[" => {
                let packet = resolve_tokens(tokens)?;
                result_stack.push(packet);
            }
            "]" => {
                let p = PacketData::List(result_stack);
                return Ok(p);
            }
            "," => {
                continue;
            }
            _ => result_stack.push(PacketData::Value(token.to_string().parse()?)),
        }
    }
}

fn compare_packets(left: &PacketData, right: &PacketData) -> CompareResult {
    match (left, right) {
        (Value(l), Value(r)) => match l.cmp(r) {
            Ordering::Less => Valid,
            Ordering::Equal => Equivalent,
            Ordering::Greater => Invalid,
        },
        (List(left), List(right)) => {
            if right.len() < left.len() {
                return Invalid;
            }

            for (left, right) in left.iter().zip(right.iter()) {
                match compare_packets(left, right) {
                    Valid => return Valid,
                    Invalid => return Invalid,
                    Equivalent => continue,
                }
            }

            Valid
        }
        (List(left), Value(_)) => {
            if let Some(first_left_val) = left.first() {
                let r = compare_packets(first_left_val, right);
                if r == Equivalent {
                    Valid
                } else {
                    r
                }
            } else {
                Valid
            }
        }
        (Value(_), List(right)) => {
            if let Some(first_right_val) = right.first() {
                let r = compare_packets(left, first_right_val);
                if r == Equivalent {
                    Valid
                } else {
                    r
                }
            } else {
                Valid
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    mod compare_packets {
        use crate::year_2022::day_13::compare_packets;
        use crate::year_2022::day_13::parse_line;
        use crate::year_2022::day_13::CompareResult::*;
        use crate::year_2022::day_13::PacketData::*;

        // #[test]
        // fn compare_lists_righ_less_values_failes() {
        //     assert!(compare_packets(
        //         &parse_line("[1]").unwrap(),
        //         &parse_line("[[2,3,4]]").unwrap()
        //     ));
        // }

        #[test]
        fn compare_list_with_value_left_success() {
            assert_eq!(
                compare_packets(
                    &parse_line("[1]").unwrap(),
                    &parse_line("[[2,3,4]]").unwrap(),
                ),
                Valid
            );
        }

        #[test]
        fn compare_list_with_value_right_success() {
            assert_eq!(
                compare_packets(
                    &parse_line("[[2,3,4]]").unwrap(),
                    &parse_line("[4]").unwrap()
                ),
                Valid
            );
        }

        #[test]
        fn compare_list_with_values_in_right_order() {
            assert_eq!(
                compare_packets(
                    &parse_line("[1,1,3,1,1]").unwrap(),
                    &parse_line("[1,1,5,1,1]").unwrap()
                ),
                Valid
            );
        }

        #[test]
        fn compare_list_with_values_in_right_order_then_not() {
            assert_eq!(
                compare_packets(
                    &parse_line("[1,1,3,1,7]").unwrap(),
                    &parse_line("[1,1,5,1,1]").unwrap()
                ),
                Valid
            );
        }

        #[test]
        fn compare_list_with_values_right_runs_out() {
            assert_eq!(
                compare_packets(
                    &parse_line("[7,7,7,7]").unwrap(),
                    &parse_line("[7,7,7]").unwrap()
                ),
                Invalid
            );
        }

        #[test]
        fn compare_pair_3_invalid() {
            assert_eq!(
                compare_packets(
                    &parse_line("[9]").unwrap(),
                    &parse_line("[[8,7,6]]").unwrap()
                ),
                Invalid
            );
        }

        #[test]
        fn compare_empty_nested_lists_right_runs_out() {
            assert_eq!(
                compare_packets(&parse_line("[[[]]]").unwrap(), &parse_line("[[]]").unwrap()),
                Invalid
            );
        }

        #[test]
        fn compare_list_right_contains_higher_value() {
            assert_eq!(
                compare_packets(
                    &parse_line("[5,6,7]").unwrap(),
                    &parse_line("[5,6,0]").unwrap()
                ),
                Invalid
            );
        }

        #[test]
        fn compare_empty_list_with_filled() {
            assert_eq!(
                compare_packets(&List(Vec::new()), &List(vec![Value(3)])),
                Valid
            );
        }

        #[test]
        fn equal_values_are_equivalent() {
            assert_eq!(compare_packets(&Value(1), &Value(1)), Equivalent);
        }

        #[test]
        fn lower_left_value_returns_true() {
            assert_eq!(compare_packets(&Value(1), &Value(2)), Valid);
        }

        #[test]
        fn higher_left_value_returns_false() {
            assert_eq!(compare_packets(&Value(2), &Value(1)), Invalid);
        }
    }

    mod parse_line {
        use crate::year_2022::day_13::PacketData::*;

        use super::parse_line;

        #[test]
        fn packet_with_multi_character_number() {
            let result = parse_line("[10]");
            let expected = List(vec![Value(10)]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
        #[test]
        fn packet_with_mixed_values_and_lists() {
            let result = parse_line("[1,[9,8],3,[1,2]]");
            let expected = List(vec![
                Value(1),
                List(vec![Value(9), Value(8)]),
                Value(3),
                List(vec![Value(1), Value(2)]),
            ]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }

        #[test]
        fn packet_with_multiple_nested_value_packet() {
            let result = parse_line("[[9,8],[1,2]]");
            let expected = List(vec![
                List(vec![Value(9), Value(8)]),
                List(vec![Value(1), Value(2)]),
            ]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }

        #[test]
        fn packet_with_multiple_nested_empty_packet() {
            let result = parse_line("[[],[]]");
            let expected = List(vec![List(Vec::new()), List(Vec::new())]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }

        #[test]
        fn packet_with_nested_empty_packet() {
            let result = parse_line("[[]]");
            let expected = List(vec![List(Vec::new())]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }

        #[test]
        fn packet_with_multiple_values_success() {
            let result = parse_line("[9,1,3]");
            let expected = List(vec![Value(9), Value(1), Value(3)]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }

        #[test]
        fn packet_with_single_value_success() {
            let result = parse_line("[9]");
            let expected = List(vec![Value(9)]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }

        #[test]
        fn empty_line_fails() {
            let result = parse_line("");

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "Line is empty");
        }

        #[test]
        fn empty_packet_success() {
            let expected = List(Vec::new());

            let result = parse_line("[]");

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }

        #[test]
        fn line_not_ending_with_brace_fails() {
            let result = parse_line("[0,9,1");

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "Line does not end with ]");
        }

        #[test]
        fn line_not_starting_with_brace_fails() {
            let result = parse_line("0,9,1]");

            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                "Line does not start with ["
            );
        }
    }

    #[test]
    fn solve_part_one_example() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(result.unwrap(), 13);
    }
}
