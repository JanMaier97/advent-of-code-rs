use linkme::distributed_slice;

use crate::{MyResult, SolverMetadata, SOLVERS};

use super::{compute_solution, Operator};

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 7,
    part: 1,
    func: solve,
    input: super::INPUT,
};

fn solve(input: &str) -> MyResult<u64> {
    compute_solution(input, &[Operator::Add, Operator::Multiply])
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 3749);
    }
}
