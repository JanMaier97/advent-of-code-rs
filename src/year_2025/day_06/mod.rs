mod part_1;
mod part_2;

static INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy)]
enum Operator {
    Multiply,
    Add,
}

struct MathProblem {
    numbers: Vec<u64>,
    operation: Operator,
}

fn solve_problem(problem: MathProblem) -> u64 {
    let operator = problem.operation;
    problem
        .numbers
        .into_iter()
        .reduce(|acc, e| match operator {
            Operator::Multiply => acc * e,
            Operator::Add => acc + e,
        })
        .unwrap_or(0)
}
