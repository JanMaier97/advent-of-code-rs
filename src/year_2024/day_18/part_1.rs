use std::collections::{HashMap, HashSet};

use anyhow::Result;
use macros::aoc_solver;

use crate::common::math_2d::{Dimensions, Point};

use super::{bfs_search, parse_input};

#[aoc_solver(2024, 18, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    solve_for_inputs(
        input,
        1024,
        Dimensions {
            width: 71,
            height: 71,
        },
    )
}

fn solve_for_inputs(input: &str, bytes_to_apply: usize, dim: Dimensions) -> Result<String> {
    let all_bytes = parse_input(input)?;
    let corrupted: HashSet<Point<i32>> = all_bytes.iter().take(bytes_to_apply).cloned().collect();
    let start = HashSet::from([Point::new(0, 0)]);

    let path = bfs_search(&corrupted, HashSet::new(), &start, HashMap::new(), dim);

    let steps = path.len() - 1;

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use crate::common::math_2d::Dimensions;

    #[test]
    fn solve_example() {
        let result = super::solve_for_inputs(
            include_str!("example.txt"),
            12,
            Dimensions {
                width: 7,
                height: 7,
            },
        )
        .unwrap();
        assert_eq!(result, "22");
    }
}
