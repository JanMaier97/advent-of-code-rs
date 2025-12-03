use macros::aoc_solver;

use anyhow::Result;

static INPUT: &str = include_str!("input.txt");

#[aoc_solver(2025, 3, 1, INPUT)]
pub fn solve(input: &str) -> Result<String> {
    let sum = input
        .lines()
        .map(|l| get_joltage(l))
        .sum::<Result<u64>>()?;

    Ok(sum.to_string())
}

fn get_joltage(line: &str) -> Result<u64> {
    let (idx, first_digit) = line
        .chars()
        .take(line.len() - 1)
        .enumerate()
        .reduce(|(lidx, l), (ridx, r)| if r > l { (ridx, r) } else { (lidx, l)})
        .unwrap();

    let second_digit = line
        .chars()
        .skip(idx+1)
        .max()
        .unwrap();

    let first_digit = first_digit.to_digit(10).unwrap();
    let second_digit = second_digit.to_digit(10).unwrap();

    return Ok((first_digit * 10 + second_digit).into());
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "357");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve(super::INPUT).unwrap();
        assert_eq!(result, "17346");
    }
}
