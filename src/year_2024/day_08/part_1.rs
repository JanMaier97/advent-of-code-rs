use std::collections::HashSet;

use macros::aoc_solver;

use crate::year_2024::day_08::parse_map;

use super::Point2;

use anyhow::Result;

#[aoc_solver(2024, 8, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let map = parse_map(input)?;
    let count = map
        .frequencies
        .iter()
        .flat_map(|(_, points)| find_antinodes(points))
        .filter(|point| map.dim.is_in_bounds(*point))
        .collect::<HashSet<_>>()
        .len();

    let count = u64::try_from(count)?;

    Ok(count.to_string())
}

fn find_antinodes(frequency_origins: &HashSet<Point2>) -> HashSet<Point2> {
    let mut antinode_positions = HashSet::new();
    let pairs = find_node_pairs(frequency_origins);

    for (left, right) in pairs.iter().cloned() {
        let vec = left.to(right);
        antinode_positions.insert(right + vec);
        antinode_positions.insert(left - vec);
    }

    antinode_positions
}

fn find_node_pairs(points: &HashSet<Point2>) -> HashSet<(Point2, Point2)> {
    let points = Vec::from_iter(points.iter().cloned());
    let mut pairs = HashSet::new();
    for (idx, left) in points.iter().enumerate() {
        for right in points.iter().skip(idx + 1) {
            pairs.insert((*left, *right));
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "14");
    }
}
