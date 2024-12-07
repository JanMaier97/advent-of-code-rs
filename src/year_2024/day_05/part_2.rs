use linkme::distributed_slice;

use crate::{MyResult, SolverMetadata, SOLVERS};

use super::{is_job_correct, parse_input, Page, PageRules, PrintJob};

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 5,
    part: 2,
    func: solve,
    input: super::INPUT,
};

fn solve(input: &str) -> MyResult<u64> {
    let data = parse_input(input)?;
    let result: u32 = data
        .jobs
        .iter()
        .filter(|job| !is_job_correct(job, &data.rules))
        .map(|job| correct_job(job, &data.rules))
        .map(|pages| pages[pages.len() / 2].0)
        .sum();
    Ok(result.into())
}

fn correct_job(job: &PrintJob, rules: &PageRules) -> Vec<Page> {
    merge_sort(&job.pages, rules)
}

fn merge_sort(pages: &[Page], rules: &PageRules) -> Vec<Page> {
    if pages.len() == 1 {
        return pages.to_vec();
    }

    let middle = pages.len() / 2;
    let left = merge_sort(&pages[..middle], rules);
    let right = merge_sort(&pages[middle..], rules);

    merge(&left, &right, rules)
}

fn merge(left: &[Page], right: &[Page], rules: &PageRules) -> Vec<Page> {
    let mut merged = Vec::new();
    let mut left_idx = 0;
    let mut right_idx = 0;

    for _ in 0..(left.len() + right.len()) {
        if right_idx < right.len()
            && (left_idx >= left.len() || page_is_before(&right[right_idx], &left[left_idx], rules))
        {
            merged.push(right[right_idx]);
            right_idx += 1;
        } else {
            merged.push(left[left_idx]);
            left_idx += 1;
        }
    }

    merged
}

fn page_is_before(page: &Page, other: &Page, rules: &PageRules) -> bool {
    rules.get(page).unwrap().pages_after.contains(other)
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve_exampe() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 123);
    }
}
