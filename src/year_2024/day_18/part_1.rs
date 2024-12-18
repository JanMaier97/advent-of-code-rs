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
    let all_bytes = parse_input(input, bytes_to_apply)?;
    let corrupted: HashSet<Point<i32>> = all_bytes.iter().take(bytes_to_apply).cloned().collect();
    let start = HashSet::from([Point::new(0, 0)]);

    let path = bfs_search(&corrupted, HashSet::new(), &start, HashMap::new(), dim);
    
    let steps = path.len()-1;

    Ok(steps.to_string())
}

fn parse_input(input: &str, count: usize) -> Result<Vec<Point<i32>>> {
    let mut points = Vec::new();
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

        points.push(Point::new(x, y));
    }

    Ok(points)
}

fn bfs_search(
    corrupted: &HashSet<Point<i32>>,
    mut visited: HashSet<Point<i32>>,
    to_visit: &HashSet<Point<i32>>,
    mut parent_map: HashMap<Point<i32>,Point<i32>>,
    dim: Dimensions,
) -> Vec<Point<i32>> {
    let _ = parent_map;
    let goal = Point::new((dim.width - 1) as i32, (dim.height - 1) as i32);

    let mut next_to_visit: HashSet<Point<i32>> = HashSet::new();
    for point in to_visit {
        let neighbours = get_next_points(*point, dim)
            .into_iter()
            .filter(|p| !visited.contains(p) && !corrupted.contains(p))
            .collect_vec();

        for n in neighbours {
            parent_map.insert(n, *point);
            next_to_visit.insert(n);

            if n == goal {
                return collect_path(parent_map, goal);
            }
        }
    } 

    visited.extend(to_visit);

    bfs_search(corrupted, visited, &next_to_visit, parent_map, dim)
}

fn collect_path(parent_map: HashMap<Point<i32>, Point<i32>>, goal: Point<i32>) -> Vec<Point<i32>> {
    let mut path = vec![goal];

    loop {
        let child = path.last().unwrap();
        let Some(parent) = parent_map.get(child) else {
            break;
        };
        path.push(*parent);
    }

    path.reverse();

    path
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
