use linkme::distributed_slice;

use crate::{MyResult, SolverMetadata, SOLVERS};

use super::{all_levels_safe, parse_and_count};


#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {year: 2024, day: 2, part: 1, func: solve, input: super::INPUT };

fn solve(input: &str) -> MyResult<u32> {
    parse_and_count(input, all_levels_safe)
}


#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn solve_example() {
        let result = super::solve(EXAMPLE).unwrap();
        assert_eq!(result, 2);
    }
}