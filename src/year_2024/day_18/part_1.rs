use std::collections::{HashMap, HashSet};

use anyhow::{bail, Result};
use itertools::Itertools;
use macros::aoc_solver;

use crate::common::math_2d::{Dimensions, Point, Vec2};

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
    let corrupted = parse_input(input, bytes_to_apply)?;
    let start = HashSet::from([Point::new(0, 0)]);
    let steps = bfs_search(&corrupted, HashSet::new(), &start, dim, 0);

    Ok(steps.to_string())
}

fn parse_input(input: &str, count: usize) -> Result<HashSet<Point<i32>>> {
    let mut points = HashSet::new();
    for (idx, line) in input.lines().enumerate() {
        if idx == count {
            break;
        }

        let split = line.split(',').collect_vec();
        if split.len() != 2 {
            bail!("Invalid point on line {}", idx);
        }

        let x = split[0].parse::<i32>()?;
        let y = split[1].parse::<i32>()?;

        points.insert(Point::new(x, y));
    }

    Ok(points)
}

fn bfs_search(
    corrupted: &HashSet<Point<i32>>,
    visited: HashSet<Point<i32>>,
    to_visit: &HashSet<Point<i32>>,
    dim: Dimensions,
    step: u64,
) -> u64 {
    let goal = Point::new((dim.width - 1) as i32, (dim.height - 1) as i32);
    if to_visit.contains(&goal) {
        return step;
    }

    let next_to_visit: HashSet<_> = to_visit
        .iter()
        .flat_map(|p| get_next_points(*p, dim))
        .filter(|p| !visited.contains(p) && !corrupted.contains(p))
        .collect();

    let visited = visited.union(to_visit).cloned().collect();

    bfs_search(corrupted, visited, &next_to_visit, dim, step + 1)
}

fn get_next_points(pos: Point<i32>, dim: Dimensions) -> HashSet<Point<i32>> {
    let points = HashSet::from([
        pos - Vec2::UP,
        pos - Vec2::DOWN,
        pos - Vec2::RIGHT,
        pos - Vec2::LEFT,
    ]);

    points
        .into_iter()
        .filter(|p| p.x >= 0 && p.y >= 0 && (p.x as u64) < dim.width && (p.y as u64) < dim.height)
        .collect::<HashSet<Point<i32>>>()
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
