use crate::{MyResult, SolverMetadata, SOLVERS};
use linkme::distributed_slice;

use super::{is_job_correct, parse_input};

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 5,
    part: 1,
    func: solve,
    input: super::INPUT,
};

fn solve(input: &str) -> MyResult<u64> {
    let data = parse_input(input)?;
    let result: u32 = data
        .jobs
        .iter()
        .filter(|job| is_job_correct(job, &data.rules))
        .map(|job| job.pages[job.pages.len() / 2].0)
        .sum();
    Ok(result.into())
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve_exampe() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 143);
    }
}
