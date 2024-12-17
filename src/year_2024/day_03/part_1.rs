use macros::aoc_solver;
use regex::Regex;

use anyhow::Result;

#[aoc_solver(2024, 3, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let multiplications = parse_input(input)?;

    let sum: u32 = multiplications.iter().map(|(op1, op2)| op1 * op2).sum();

    Ok(sum.to_string())
}

fn parse_input(input: &str) -> Result<Vec<(u32, u32)>> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let mut operands = Vec::new();

    for line in input.lines() {
        let line_operands = regex
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [op1, op2])| parse_operands(op1, op2))
            .collect::<Result<Vec<_>, _>>()?;

        operands.extend(line_operands);
    }

    Ok(operands)
}

fn parse_operands(op1: &str, op2: &str) -> Result<(u32, u32)> {
    let op1 = op1.parse::<u32>()?;
    let op2 = op2.parse::<u32>()?;
    Ok((op1, op2))
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example1.txt");

    #[test]
    fn solve_example() {
        let result = super::solve(EXAMPLE).unwrap();
        assert_eq!(result, "161");
    }
}
