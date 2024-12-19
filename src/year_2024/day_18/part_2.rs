use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use macros::aoc_solver;

use crate::common::math_2d::{Dimensions, Point};

use super::{bfs_search, parse_input};

#[aoc_solver(2024, 18, 2, super::INPUT)]
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

    let corrupted = all_bytes.iter().take(bytes_to_apply).cloned().collect();
    let path = get_inital_path(corrupted, dim);

    let point = find_first_blocking_byte(&path, &all_bytes, bytes_to_apply, dim)
        .ok_or(anyhow!("The path is never blocked"))?;

    Ok(format!("{},{}", point.x, point.y))
}

fn find_first_blocking_byte(
    path: &[Point<i32>],
    all_bytes: &[Point<i32>],
    count: usize,
    dim: Dimensions,
) -> Option<Point<i32>> {
    let mut current_path = path.to_vec();
    let mut corrupted: HashSet<_> = all_bytes.iter().take(count).cloned().collect();
    let remaining_bytes = all_bytes.iter().skip(count).collect_vec();

    for byte in remaining_bytes {
        corrupted.insert(*byte);

        if current_path.contains(byte) {
            current_path = find_new_path_simple(&corrupted, dim);
            if current_path.is_empty() {
                return Some(*byte);
            }
        }
    }

    None
}

// fn find_new_path(path: Vec<Point<i32>>, byte: Point<i32>, corrupted: &HashSet<Point<i32>>, dim: Dimensions) -> Vec<Point<i32>> {
//     let (blocked_idx, _) =  path.iter().find_position(|p| **p == byte).unwrap();
//     let mut new_start = path[0..blocked_idx].to_vec();
//     let to_visit = HashSet::from([*new_start.last().unwrap()]);
//     let new_path = bfs_search(corrupted, HashSet::new(), &to_visit, HashMap::new(), dim);
// }

fn find_new_path_simple(corrupted: &HashSet<Point<i32>>, dim: Dimensions) -> Vec<Point<i32>> {
    let start = HashSet::from([Point::new(0, 0)]);
    bfs_search(corrupted, HashSet::new(), &start, HashMap::new(), dim)
}

fn get_inital_path(bytes: HashSet<Point<i32>>, dim: Dimensions) -> Vec<Point<i32>> {
    let start = HashSet::from([Point::new(0, 0)]);

    bfs_search(&bytes, HashSet::new(), &start, HashMap::new(), dim)
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
        assert_eq!(result, "6,1");
    }
}
