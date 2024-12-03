use linkme::distributed_slice;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{MyResult, SolverMetadata, SOLVERS};

const DO_NOT_PATTERN: &str = r"don't\(\)";
const DO_PATTERN: &str = r"do\(\)";
const MULTIPLY_PATTERN: &str = r"mul\((\d{1,3}),(\d{1,3})\)";

enum Operator {
    Do,
    DoNot,
    Multiply(u32, u32),
}

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 3,
    part: 2,
    func: solve,
    input: super::INPUT,
};
fn solve(input: &str) -> MyResult<u32> {
    let operators = parse_input(input)?;

    let mut sum = 0;
    let mut is_multiply_enabled = true;
    for operator in operators {
        match operator {
            Operator::Do => is_multiply_enabled = true,
            Operator::DoNot => is_multiply_enabled = false,
            Operator::Multiply(a, b) => {
                if is_multiply_enabled {
                    sum += a * b
                }
            }
        }
    }

    Ok(sum)
}

fn parse_input(input: &str) -> MyResult<Vec<Operator>> {
    let literal_regex =
        Regex::new(format!("{DO_PATTERN}|{DO_NOT_PATTERN}|{MULTIPLY_PATTERN}").as_str())?;
    let mut operators = Vec::new();

    for line in input.lines() {
        let line_ops = literal_regex
            .find_iter(line)
            .map(|c| c.as_str())
            .map(|literal| parse_literal(literal))
            .collect::<Result<Vec<_>, _>>()?;

        operators.extend(line_ops);
    }

    Ok(operators)
}

fn parse_literal(literal: &str) -> MyResult<Operator> {
    static DO_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(DO_PATTERN).unwrap());
    static DO_NOT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(DO_NOT_PATTERN).unwrap());

    if DO_REGEX.is_match(literal) {
        return Ok(Operator::Do);
    }

    if DO_NOT_REGEX.is_match(literal) {
        return Ok(Operator::DoNot);
    }

    parse_multiplier(literal)
}

fn parse_multiplier(literal: &str) -> MyResult<Operator> {
    static MULT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(MULTIPLY_PATTERN).unwrap());

    let Some(capture) = MULT_REGEX.captures(literal) else {
        return Err(format!("Found Invalid operand: '{}'", literal).into());
    };

    let (_, [op1, op2]) = capture.extract();

    let op1 = op1.parse::<u32>()?;
    let op2 = op2.parse::<u32>()?;

    Ok(Operator::Multiply(op1, op2))
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example2.txt");

    #[test]
    fn solve_example() {
        let result = super::solve(EXAMPLE).unwrap();
        assert_eq!(result, 48);
    }
}
