use macros::aoc_solver;

use anyhow::{bail, ensure, Result};

use super::{solve_problem, MathProblem, Operator, INPUT};

struct ColumnInfo {
    operator: Operator,
    width: usize,
}

#[aoc_solver(2025, 6, 2, INPUT)]
fn solve_part_2(input: &str) -> Result<String> {
    let problems = parse_input(input)?;
    let result: u64 = problems.into_iter().map(|p| solve_problem(p)).sum();
    Ok(result.to_string())
}

fn parse_input(input: &str) -> Result<Vec<MathProblem>> {
    let mut token_grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.starts_with('+') || line.starts_with('*') {
            let column_infos = parse_column_info(line)?;

            return build_problems(&column_infos, &token_grid);
        }
        token_grid.push(line.chars().collect());
    }

    bail!("Input does not contain any operators");
}

fn build_problems(
    column_infos: &[ColumnInfo],
    token_grid: &[Vec<char>],
) -> Result<Vec<MathProblem>> {
    if token_grid.len() == 0 {
        return Ok(Vec::new());
    }

    ensure!(
        token_grid
            .iter()
            .all(|row| row.len() == token_grid[0].len()),
        "Failed to correctly parse the math problems"
    );

    let mut math_problems = Vec::with_capacity(column_infos.len());
    let mut column_offset = 0;
    for column_info in column_infos.iter() {
        let mut numbers = Vec::new();
        let last_number_column = column_offset + column_info.width;
        for column_idx in column_offset..last_number_column {
            let number = token_grid
                .iter()
                .map(|row| row[column_idx])
                .filter(|c| *c != ' ')
                .collect::<String>()
                .parse::<u64>()?;
            numbers.push(number);
        }
        math_problems.push(MathProblem {
            numbers,
            operation: column_info.operator,
        });
        column_offset = last_number_column + 1;
    }

    Ok(math_problems)
}

fn parse_column_info(line: &str) -> Result<Vec<ColumnInfo>> {
    let mut columns = Vec::new();
    let mut current_operator = if line.starts_with('+') {
        Operator::Add
    } else {
        Operator::Multiply
    };
    let mut width = 1;
    for token in line.chars().skip(1) {
        if token == '+' || token == '*' {
            columns.push(ColumnInfo {
                operator: current_operator,
                width: width - 1,
            });
            width = 0;
            current_operator = if token == '+' {
                Operator::Add
            } else {
                Operator::Multiply
            };
        }
        width += 1;
    }
    columns.push(ColumnInfo {
        operator: current_operator,
        width,
    });
    Ok(columns)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_2() {
        let result = super::solve_part_2(include_str!("example.txt")).unwrap();
        assert_eq!(result, "3263827");
    }

    #[test]
    fn solve_part_2() {
        let result = super::solve_part_2(super::INPUT).unwrap();
        assert_eq!(result, "11950004808442");
    }
}
