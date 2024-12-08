use std::collections::HashSet;

use linkme::distributed_slice;

use crate::{year_2024::day_08::parse_map, MyResult, SolverMetadata, SOLVERS};

use super::{Dimensions, Point2, Vec2};

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 8,
    part: 2,
    func: solve,
    input: super::INPUT,
};

fn solve(input: &str) -> MyResult<u64> {
    let map = parse_map(input)?;
    let count = map
        .frequencies
        .iter()
        .flat_map(|(_, points)| find_antinodes(points, map.dim))
        .collect::<HashSet<_>>()
        .len();

    let count = u64::try_from(count)?;

    Ok(count)
}

fn find_antinodes(frequency_origins: &HashSet<Point2>, dim: Dimensions) -> HashSet<Point2> {
    let mut antinode_positions = HashSet::new();
    let pairs = find_node_pairs(frequency_origins);

    for (left, right) in pairs.iter().cloned() {
        let vec = left.to(right);
        antinode_positions.extend(find_antinodes_on_line(right, vec, dim).iter());
        antinode_positions.extend(find_antinodes_on_line(left, vec * -1, dim).iter());
    }

    antinode_positions
}

fn find_antinodes_on_line(origin: Point2, direction: Vec2, dim: Dimensions) -> HashSet<Point2> {
    let mut points = HashSet::new();
    for scarlar in 0.. {
        let point = origin + direction * scarlar;

        if !dim.is_in_bounds(point) {
            break;
        }

        points.insert(point);
    }

    points
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
        assert_eq!(result, 34);
    }
}
