use std::collections::HashMap;

use macros::aoc_solver;

use crate::year_2024::day_11::parse_input;

use anyhow::Result;

#[aoc_solver(2024, 11, 2, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let values = parse_input(input)?;
    let count = blink(75, values);
    Ok(count.to_string())
}

fn blink(times: usize, values: Vec<u64>) -> u64 {
    values
        .iter()
        .map(|value| count_recursively(*value, times, &mut HashMap::new()))
        .sum()
}

fn count_recursively(
    value: u64,
    blink_count: usize,
    results: &mut HashMap<(u64, usize), u64>,
) -> u64 {
    if let Some(count) = results.get(&(value, blink_count)) {
        return *count;
    }

    if blink_count == 0 {
        results.insert((value, blink_count), 1);
        return 1;
    }

    if value == 0 {
        let count = count_recursively(1, blink_count - 1, results);
        results.insert((value, blink_count), count);
        return count;
    }

    let digit_count = count_digits(value);
    if digit_count % 2 == 0 {
        let middle = digit_count / 2;
        let div = 10_u64.pow(middle as u32);

        let count1 = count_recursively(value / div, blink_count - 1, results);
        let count2 = count_recursively(value % div, blink_count - 1, results);
        results.insert((value, blink_count), count1 + count2);
        return count1 + count2;
    }

    let count = count_recursively(value * 2024, blink_count - 1, results);
    results.insert((value, blink_count), count);
    count
}

fn count_digits(value: u64) -> usize {
    if value == 0 {
        return 1;
    }

    let mut count = 0;
    let mut value = value;
    while value > 0 {
        value /= 10;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_11::part_2::count_digits;

    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "65601038650482");
    }

    #[test]
    fn digit_count_correct() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(110), 3);
    }
}
