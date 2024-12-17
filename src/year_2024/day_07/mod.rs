use std::collections::HashSet;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use anyhow::{bail, Result};

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Ord, PartialOrd)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

struct Equation {
    operands: Vec<u64>,
    result: u64,
}

fn compute_solution(input: &str, operators: &[Operator]) -> Result<String> {
    let equations = parse_input(input)?;

    let result: u64 = equations
        .par_iter()
        .filter(|eq| is_equation_valid(eq, operators).unwrap())
        .map(|eq| eq.result)
        .sum();

    Ok(result.to_string())
}

fn is_equation_valid(equation: &Equation, operators: &[Operator]) -> Result<bool> {
    let operator_permutations: HashSet<Vec<Operator>> =
        generate_operand_permutations(operators, equation.operands.len() - 1);

    assert_eq!(
        operator_permutations.len(),
        operators.len().pow(equation.operands.len() as u32 - 1)
    );

    for operators in operator_permutations.iter() {
        let result = compute_result(&equation.operands, operators)?;
        if equation.result == result {
            return Ok(true);
        }
    }

    Ok(false)
}

fn compute_result(operands: &[u64], operators: &[Operator]) -> Result<u64> {
    if operands.len() != operators.len() + 1 {
        bail!("Invalid amount of operators for the number of operands");
    }

    let mut result = operands[0];
    for (b, operator) in operands.iter().skip(1).zip(operators) {
        result = apply_operator(result, *b, *operator);
    }

    Ok(result)
}

fn apply_operator(a: u64, b: u64, operator: Operator) -> u64 {
    match operator {
        Operator::Add => a.checked_add(b).unwrap(),
        Operator::Multiply => a.checked_mul(b).unwrap(),
        Operator::Concatenate => format!("{}{}", a, b).parse::<u64>().unwrap(),
    }
}

fn generate_operand_permutations(operators: &[Operator], count: usize) -> HashSet<Vec<Operator>> {
    let mut permutations = HashSet::new();

    if count == 0 {
        permutations.insert(Vec::new());
        return permutations;
    }

    let base_permutation = generate_operand_permutations(operators, count - 1);

    for operator in operators {
        for permutation in base_permutation.iter() {
            let mut new_permutation = permutation.clone();
            new_permutation.push(*operator);
            permutations.insert(new_permutation);
        }
    }

    permutations
}

fn parse_input(input: &str) -> Result<Vec<Equation>> {
    input.lines().map(parse_equation).try_collect()
}

fn parse_equation(line: &str) -> Result<Equation> {
    let split: Vec<&str> = line.split(": ").collect();
    if split.len() != 2 {
        bail!("Invalid equation: {}", line);
    }

    let result: u64 = split[0].parse()?;
    let operands: Vec<u64> = split[1].split(' ').map(|op| op.parse()).try_collect()?;
    let equation = Equation { result, operands };

    Ok(equation)
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_07::{is_equation_valid, Equation, Operator};

    #[test]
    fn test_individual_equations() {
        let ops = &[Operator::Add, Operator::Multiply];
        let eq = Equation {
            result: 190,
            operands: vec![10, 19],
        };
        assert!(is_equation_valid(&eq, ops).unwrap());

        let eq = Equation {
            result: 3267,
            operands: vec![81, 40, 27],
        };
        assert!(is_equation_valid(&eq, ops).unwrap());

        let eq = Equation {
            result: 83,
            operands: vec![17, 5],
        };
        assert!(!is_equation_valid(&eq, ops).unwrap());

        let eq = Equation {
            result: 156,
            operands: vec![15, 6],
        };
        assert!(!is_equation_valid(&eq, ops).unwrap());

        let eq = Equation {
            result: 7290,
            operands: vec![6, 8, 6, 15],
        };
        assert!(!is_equation_valid(&eq, ops).unwrap());

        let eq = Equation {
            result: 161011,
            operands: vec![16, 10, 13],
        };
        assert!(!is_equation_valid(&eq, ops).unwrap());

        let eq = Equation {
            result: 192,
            operands: vec![17, 8, 14],
        };
        assert!(!is_equation_valid(&eq, ops).unwrap());

        let eq = Equation {
            result: 21037,
            operands: vec![9, 7, 18, 13],
        };
        assert!(!is_equation_valid(&eq, ops).unwrap());

        let eq = Equation {
            result: 292,
            operands: vec![11, 6, 16, 20],
        };
        assert!(is_equation_valid(&eq, ops).unwrap());
    }
}
