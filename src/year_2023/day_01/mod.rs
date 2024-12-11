use std::{cmp::max, collections::HashSet};

use macros::aoc_solver;

use crate::MyResult;

const INPUT: &str = include_str!("input.txt");
const SPELLED_DIGIT: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug, Hash, Eq)]
struct Digit {
    value: u64,
    index: usize,
}

impl PartialEq for Digit {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.index == other.index
    }
}

#[aoc_solver(2023, 1, 1, INPUT)]
fn solve_part_one(input: &str) -> MyResult<u64> {
    compute_value(input, find_raw_digits)
}

#[aoc_solver(2023, 1, 2, INPUT)]
fn solve_part_two(input: &str) -> MyResult<u64> {
    compute_value(input, find_raw_and_spelled_digits)
}

fn compute_value<F>(input: &str, digit_finder: F) -> MyResult<u64>
where
    F: Fn(&str) -> Vec<Digit>,
{
    let sum = input
        .lines()
        .map(|l| digit_finder(l))
        .map(|digits| compute_value_of_digits(&digits))
        .collect::<MyResult<Vec<_>>>()?
        .into_iter()
        .sum();

    Ok(sum)
}

fn find_raw_and_spelled_digits(line: &str) -> Vec<Digit> {
    let mut digits = find_raw_digits(line);

    digits.extend(find_spelled_digits(line));
    digits.sort_by(|a, b| Ord::cmp(&a.index, &b.index));

    digits
}

fn compute_value_of_digits(digits: &[Digit]) -> MyResult<u64> {
    let first_digit = digits.first().ok_or("No digits found in line")?;
    let last_digit = digits.last().ok_or("No digits found in line")?;

    let value = first_digit.value * 10 + last_digit.value;
    Ok(value)
}

fn find_raw_digits(line: &str) -> Vec<Digit> {
    let res = line
        .chars()
        .map(|c| c.to_string().parse::<u64>().ok())
        .enumerate()
        .map(|(index, option)| option.map(|value| Digit { value, index }))
        .flatten()
        .collect::<Vec<_>>();

    return res;
}

fn find_spelled_digits(line: &str) -> HashSet<Digit> {
    let res = SPELLED_DIGIT
        .into_iter()
        .enumerate()
        .map(|(index, name)| (index + 1, name, sliding_windows(line, name.len())))
        .flat_map(|(value, name, windows)| collect_digits_from_windows(value, name, &windows))
        .collect::<HashSet<_>>();

    return res;
}

fn collect_digits_from_windows(
    digit_value: usize,
    digit_name: &str,
    windows: &[String],
) -> Vec<Digit> {
    windows
        .into_iter()
        .enumerate()
        .filter(|(_, window)| *window == digit_name)
        .map(|(index, _)| Digit {
            value: u64::try_from(digit_value).unwrap(),
            index,
        })
        .collect::<Vec<_>>()
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
