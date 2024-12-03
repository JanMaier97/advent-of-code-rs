use linkme::distributed_slice;
use regex::Regex;

use crate::{MyResult, SolverMetadata, SOLVERS};

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 3,
    part: 1,
    func: solve,
    input: super::INPUT,
};
fn solve(input: &str) -> MyResult<u32> {
    let multiplications = parse_input(input)?;

    let sum = multiplications.iter().map(|(op1, op2)| op1 * op2).sum();

    Ok(sum)
}

fn parse_input(input: &str) -> MyResult<Vec<(u32, u32)>> {
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

fn parse_operands(op1: &str, op2: &str) -> MyResult<(u32, u32)> {
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
        assert_eq!(result, 161);
    }
}
