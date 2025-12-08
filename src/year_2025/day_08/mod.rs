use std::collections::HashSet;

use macros::aoc_solver;

use anyhow::{bail, Context, Result};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[aoc_solver(2025, 8, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    compute_result_with_pair_count(input, 1000)
}

fn compute_result_with_pair_count(input: &str, count: usize) -> Result<String> {
    let points = parse_points(input)?;
    let sorted_pairs = get_pairs_sorted(&points);
    let mut circuits = get_circuits_sizes(&sorted_pairs[0..count])?;

    circuits.sort();

    let result = circuits
        .into_iter()
        .rev()
        .take(3)
        .reduce(|acc, e| acc * e)
        .unwrap_or(0);

    Ok(result.to_string())
}

fn get_circuits_sizes(point_pairs: &[(Point3, Point3)]) -> Result<Vec<usize>> {
    let mut circuits: Vec<HashSet<Point3>> = Vec::new();

    for (left_point, right_point) in point_pairs {
        let left_circuit_idx = circuits.iter().position(|c| c.contains(left_point));
        let right_circuit_idx = circuits.iter().position(|c| c.contains(right_point));
        match (left_circuit_idx, right_circuit_idx) {
            (None, None) => {
                let mut circuit = HashSet::new();
                circuit.insert(*left_point);
                circuit.insert(*right_point);
                circuits.push(circuit);
            }
            (None, Some(idx)) => {
                circuits[idx].insert(*left_point);
            }
            (Some(idx), None) => {
                circuits[idx].insert(*right_point);
            }
            (Some(lidx), Some(ridx)) => {
                if lidx == ridx {
                    continue;
                }
                // could be done with vec::swap_remove -> indices need to be mainted correctly then
                let points_to_merge = circuits[ridx].clone();
                circuits[lidx].extend(points_to_merge);
                circuits.remove(ridx);
            }
        }
    }

    Ok(circuits.iter().map(|c| c.len()).collect::<Vec<_>>())
}

fn get_pairs_sorted(points: &HashSet<Point3>) -> Vec<(Point3, Point3)> {
    let points = points.into_iter().collect::<Vec<_>>();
    let mut pairs_with_dist = Vec::new();
    for (idx, &&left) in points.iter().take(points.len() - 1).enumerate() {
        for &&right in points.iter().skip(idx + 1) {
            let dist = get_distance_squared(left, right);
            pairs_with_dist.push((left, right, dist));
        }
    }

    pairs_with_dist.sort_by(|(_, _, dist_a), (_, _, dist_b)| dist_a.cmp(dist_b));

    pairs_with_dist
        .into_iter()
        .map(|(l, r, _)| (l, r))
        .collect::<Vec<_>>()
}

fn get_distance_squared(a: Point3, b: Point3) -> i64 {
    (b.x - a.x).pow(2) + (b.y - a.y).pow(2) + (b.z - a.z).pow(2)
}

fn parse_points(input: &str) -> Result<HashSet<Point3>> {
    let mut points = HashSet::new();
    for line in input.lines() {
        let values = line
            .split(',')
            .map(|v| {
                v.parse::<i64>()
                    .with_context(|| format!("Failed to parse '{}' to a number", v).to_string())
            })
            .collect::<Result<Vec<_>>>()?;
        if values.len() != 3 {
            bail!("Could not parse line '{}'", line);
        }
        let point = Point3::new(values[0], values[1], values[2]);
        points.insert(point);
    }

    Ok(points)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result =
            super::compute_result_with_pair_count(include_str!("example.txt"), 10).unwrap();
        assert_eq!(result, "40");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "121770");
    }
}
