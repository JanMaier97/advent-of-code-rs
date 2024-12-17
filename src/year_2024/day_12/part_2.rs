use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;
use macros::aoc_solver;

use anyhow::Result;

use super::{collect_areas, parse_map, Area, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CornerPoint {
    x: usize,
    y: usize,
}

#[aoc_solver(2024, 12, 2, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let map = parse_map(input);
    let plants = collect_areas(&map)?;

    let sum: usize = plants
        .iter()
        .flat_map(|(_, areas)| areas.iter().map(|a| count_corners_for_area(a) * a.size()))
        .sum();

    Ok(sum.to_string())
}

fn count_corners_for_area(area: &Area) -> usize {
    area.plots
        .iter()
        .flat_map(|pos| get_corner_points(*pos))
        .collect::<HashSet<_>>()
        .iter()
        .map(|p| count_corner_point(*p, area))
        .sum()
}

fn count_corner_point(point: CornerPoint, area: &Area) -> usize {
    let plots = get_positions_for_corner(point)
        .iter()
        .cloned()
        .filter(|p| area.contains(*p))
        .collect_vec();

    let length = plots.len();

    if length == 2 && plots[0].x != plots[1].x && plots[0].y != plots[1].y {
        return 2;
    }

    if length == 3 || length == 1 {
        return 1;
    }

    0
}

fn get_corner_points(pos: Position) -> HashSet<CornerPoint> {
    HashSet::from([
        CornerPoint { x: pos.x, y: pos.y },
        CornerPoint {
            x: pos.x + 1,
            y: pos.y,
        },
        CornerPoint {
            x: pos.x,
            y: pos.y + 1,
        },
        CornerPoint {
            x: pos.x + 1,
            y: pos.y + 1,
        },
    ])
}

fn get_positions_for_corner(point: CornerPoint) -> HashSet<Position> {
    let mut positions = HashSet::from([Position {
        x: point.x,
        y: point.y,
    }]);
    if point.x > 0 {
        positions.insert(Position {
            x: point.x - 1,
            y: point.y,
        });
    }

    if point.y > 0 {
        positions.insert(Position {
            x: point.x,
            y: point.y - 1,
        });
    }

    if point.x > 0 && point.y > 0 {
        positions.insert(Position {
            x: point.x - 1,
            y: point.y - 1,
        });
    }

    positions
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_12::{collect_areas, parse_map, part_2::count_corners_for_area};

    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "1206");
    }

    #[test]
    fn test_count_corners() {
        for (input, expected) in [
            ("E", 4),
            ("EE", 4),
            ("EE\nEE", 4),
            ("EE\nEA", 6),
            ("EE\nEA\nEE", 8),
            ("EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE", 12),
        ] {
            let map = parse_map(&input);
            let plants = collect_areas(&map).unwrap();
            let area = plants.get(&'E').unwrap();
            assert_eq!(
                count_corners_for_area(&area[0]),
                expected,
                "input:\n{}",
                input
            );
        }
    }

    #[test]
    fn solve_small() {
        let result = super::solve("EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE").unwrap();
        assert_eq!(result, "236");
    }

    #[test]
    fn solve_small_2() {
        let result = super::solve("AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA").unwrap();
        assert_eq!(result, "368");
    }
}
