use macros::aoc_solver;

use anyhow::{ensure, Result};

static INPUT: &str = include_str!("input.txt");

#[aoc_solver(2025, 3, 1, INPUT)]
fn solve(input: &str) -> Result<String> {
    let sum = input
        .lines()
        .map(|l| get_voltage(l, 2))
        .sum::<Result<u64>>()?;

    Ok(sum.to_string())
}

#[aoc_solver(2025, 3, 2, INPUT)]
fn solve_part_2(input: &str) -> Result<String> {
    let sum = input
        .lines()
        .map(|l| get_voltage(l, 12))
        .sum::<Result<u64>>()?;

    Ok(sum.to_string())
}

fn get_voltage(line: &str, digit_count: usize) -> Result<u64> {
    ensure!(
        line.len() >= digit_count,
        "Line contains {} characters, but {} digits are needed.",
        line.len(),
        digit_count
    );

    if line.len() == digit_count {
        let res = line.parse::<u64>()?;
        return Ok(res);
    }

    let mut last_digit_idx: usize = 0;
    let mut digits = Vec::with_capacity(digit_count);

    for iteration in 0..digit_count {
        let search_range =
            line.len() - (last_digit_idx + iteration.min(1)) - (digit_count - iteration - 1);
        let skip = last_digit_idx + iteration.min(1);
        let (idx, digit) = line
            .chars()
            .enumerate()
            .skip(skip)
            .take(search_range)
            .reduce(|(lidx, l), (ridx, r)| if r > l { (ridx, r) } else { (lidx, l) })
            .unwrap();

        if iteration > 0 {
            ensure!(last_digit_idx < idx);
        }

        last_digit_idx = idx;
        digits.push(digit);
    }

    ensure!(digits.len() == digit_count);

    let voltage = digits.iter().collect::<String>().parse::<u64>()?;

    Ok(voltage)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "357");
    }

    #[test]
    fn get_voltate_tests() {
        let res = super::get_voltage("987654321111111", 12).unwrap();
        assert_eq!(res, 987654321111);

        let res = super::get_voltage("811111111111119", 12).unwrap();
        assert_eq!(res, 811111111119);

        let res = super::get_voltage("234234234234278", 12).unwrap();
        assert_eq!(res, 434234234278);

        let res = super::get_voltage("818181911112111", 12).unwrap();
        assert_eq!(res, 888911112111);

        let res = super::get_voltage("234234234234278", 2).unwrap();
        assert_eq!(res, 78);
    }

    #[test]
    fn solve_example_part_2() {
        let result = super::solve_part_2(include_str!("example.txt")).unwrap();
        assert_eq!(result, "3121910778619");
    }

    #[test]
    fn solve_part_2() {
        let result = super::solve_part_2(super::INPUT).unwrap();
        assert_eq!(result, "172981362045136");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve(super::INPUT).unwrap();
        assert_eq!(result, "17346");
    }
}
