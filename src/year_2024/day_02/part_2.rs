use std::collections::HashSet;

use linkme::distributed_slice;

use crate::{MyResult, SolverMetadata, SOLVERS};

use super::{all_levels_safe, parse_and_count, Record, Safety};

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 2,
    part: 2,
    func: solve,
    input: super::INPUT,
};

fn solve(input: &str) -> MyResult<u32> {
    parse_and_count(input, with_problem_dampener)
}

fn with_problem_dampener(record: &Record) -> Safety {
    let levels_to_remove = get_indices_of_unsave_levels(record);

    if levels_to_remove.len() == 0 {
        return Safety::Save;
    }

    for idx in levels_to_remove.iter() {
        let mut new_record = record.clone();
        new_record.levels.remove(*idx);

        let result = all_levels_safe(&new_record);
        if result == Safety::Save {
            return result;
        }
    }

    Safety::Unsave
}

struct WindowMap {
    indices: Vec<usize>,
    value: i32,
}

fn get_indices_of_unsave_levels(record: &Record) -> HashSet<usize> {
    let level_diffs = record
        .levels
        .windows(2)
        .enumerate()
        .map(|(idx, w)| WindowMap {
            indices: vec![idx, idx + 1],
            value: w[1] as i32 - w[0] as i32,
        })
        .collect::<Vec<_>>();

    let invalid_difference_indices = level_diffs
        .iter()
        .filter(|map| map.value.abs() < 1 || map.value.abs() > 3)
        .flat_map(|map| map.indices.clone())
        .collect::<HashSet<_>>();

    let direction_change_indicies = level_diffs
        .windows(2)
        .filter(|w| w[0].value.signum() != w[1].value.signum())
        .flat_map(|w| [w[0].indices.as_slice(), w[1].indices.as_slice()].concat())
        .collect::<HashSet<_>>();

    invalid_difference_indices
        .union(&direction_change_indicies)
        .cloned()
        .collect::<HashSet<_>>()
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn solve_example() {
        let result = super::solve(EXAMPLE).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn last_level_is_unsafe() {
        let input = "1 2 3 8";
        let result = super::solve(input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn direction_change_at_start() {
        let input = "4 2 3 4";
        let result = super::solve(input).unwrap();
        assert_eq!(result, 1);
    }
}
