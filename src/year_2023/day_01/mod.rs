use std::{cmp::max, collections::HashSet};

use itertools::Itertools;

use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");
const SPELLED_DIGIT: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug, Hash, Eq)]
struct Digit {
    value: usize,
    index: usize,
}

impl PartialEq for Digit {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.index == other.index
    }
}

pub fn solve() -> MyResult<()> {
    print_challange_header(1);

    let part_one_result = solve_part_one(INPUT)?;
    println!("Result for part one is {}", part_one_result);

    let part_two_result = solve_part_two(INPUT)?;
    println!("Result for part two is {}", part_two_result);

    Ok(())
}

fn solve_part_one(input: &str) -> MyResult<usize> {
    Ok(compute_value(input, find_raw_digits))
}

fn solve_part_two(input: &str) -> MyResult<usize> {
    Ok(compute_value(input, find_raw_and_spelled_digits))
}

fn compute_value<F>(input: &str, digit_finder: F) -> usize 
where F : Fn(&str) -> Vec<Digit>
{
    let sum = input
        .lines()
        .map(|l| digit_finder(l))
        .fold(0, |sum, digit| sum + compute_value_of_digits(&digit));

    sum
}

fn find_raw_and_spelled_digits(line: &str) -> Vec<Digit> {
        let mut digits = find_raw_digits(line);

        digits.extend(find_spelled_digits(line));
        digits.sort_by(|a, b| Ord::cmp(&a.index, &b.index));

        digits
}

fn compute_value_of_digits( digits: &[Digit]) -> usize {
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();

        first_digit.value * 10 + last_digit.value
}

fn find_raw_digits(line: &str) -> Vec<Digit> {
    let res = line
        .chars()
        .map(|c| c.to_string().parse::<usize>().ok())
        .enumerate()
        .map(|(index, option)| option.map(|value| Digit { value, index }))
        .flatten()
        .collect::<Vec<_>>();

    return res;
}

fn find_spelled_digits(line: &str) -> HashSet<Digit> {
    let mut res = HashSet::new();

    for (digit_value, digit_name) in SPELLED_DIGIT.into_iter().enumerate() {
        let windows = sliding_windows(line, digit_name.len());
        for (index, window) in windows.into_iter().enumerate() {
            if window == digit_name {
                res.insert(Digit {
                    value: digit_value + 1,
                    index,
                });
            }
        }
    }

    return res;
}

fn sliding_windows(line: &str, window_size: usize) -> Vec<String> {
    let window_count = line.len().saturating_sub(window_size) + 1;
    let window_count = max(window_count, 1);

    let mut windows = Vec::new();
    for window_index in 0..window_count {
        let window = line
            .chars()
            .skip(window_index)
            .take(window_size)
            .collect::<String>();
        windows.push(window);
    }

    windows
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::year_2023::day_01::find_spelled_digits;
    use crate::year_2023::day_01::solve_part_two;
    use crate::year_2023::day_01::Digit;

    use super::sliding_windows;
    use super::solve_part_one;
    use super::INPUT;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");
    const EXAMPLE_2_INPUT: &str = include_str!("example2.txt");

    #[test]
    fn test_part_one_example() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_part_one_input() {
        let result = solve_part_one(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 54708);
    }

    #[test]
    fn test_part_two_example() {
        let result = solve_part_two(EXAMPLE_2_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 281);
    }

    #[test]
    fn test_part_two_input() {
        let result = solve_part_two(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 54087);
    }

    #[test]
    fn find_spelled_digits_correct() {
        let input = "dqfournine5four2jmlqcgv";
        let result = find_spelled_digits(input);

        let expected_values = vec![
            Digit { value: 4, index: 2 },
            Digit { value: 9, index: 6 },
            Digit {
                value: 4,
                index: 11,
            },
        ];
        let expected = HashSet::from_iter(expected_values);

        assert_eq!(result, expected);
    }

    #[test]
    fn sliding_windows_correct_with_string_shorter_than_window_size() {
        let input = "abc";
        let expected = vec!["abc"];

        let windows = sliding_windows(input, 4);

        assert_eq!(windows, expected);
    }

    #[test]
    fn sliding_windows_correct() {
        let input = "dqfournine5four2jmlqcgv";
        let expected = vec![
            "dqfo", "qfou", "four", "ourn", "urni", "rnin", "nine", "ine5", "ne5f", "e5fo", "5fou",
            "four", "our2", "ur2j", "r2jm", "2jml", "jmlq", "mlqc", "lqcg", "qcgv",
        ];
        let windows = sliding_windows(input, 4);

        assert_eq!(windows, expected);
    }
}
