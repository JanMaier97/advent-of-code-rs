use macros::aoc_solver;

use anyhow::{bail, ensure, Context, Result};

use super::{solve_problem, MathProblem, Operator, INPUT};

#[aoc_solver(2025, 6, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    let problems = parse_input(input)?;
    let result: u64 = problems.into_iter().map(|p| solve_problem(p)).sum();
    Ok(result.to_string())
}

fn parse_input(input: &str) -> Result<Vec<MathProblem>> {
    let mut rows_of_numbers: Vec<Vec<u64>> = Vec::new();

    for line in input.lines() {
        if line.starts_with("+") || line.starts_with("*") {
            ensure!(
                rows_of_numbers.len() > 0,
                "The are now rows of values for the math problems."
            );

            let problem_count = rows_of_numbers[0].len();

            ensure!(
                rows_of_numbers.iter().all(|row| row.len() == problem_count),
                "Not all rows have the same amount of values for their math problems"
            );

            let mut problems: Vec<MathProblem> = Vec::with_capacity(problem_count);
            for (idx, c) in line.split(' ').filter(|s| *s != "").enumerate() {
                let operation = if c == "+" {
                    Operator::Add
                } else {
                    Operator::Multiply
                };
                let problem = MathProblem {
                    operation,
                    numbers: rows_of_numbers.iter().map(|row| row[idx]).collect(),
                };
                problems.push(problem)
            }

            return Ok(problems);
        }

        let values: Vec<u64> = line
            .split(' ')
            .filter(|s| *s != "")
            .map(|s| {
                s.parse::<u64>()
                    .with_context(|| format!("failed to parse value '{}'", s))
            })
            .collect::<Result<Vec<_>>>()?;

        rows_of_numbers.push(values);
    }

    bail!("Input does not contain any operators");
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve_part_1(include_str!("example.txt")).unwrap();
        assert_eq!(result, "4277556");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "6299564383938");
    }
}
