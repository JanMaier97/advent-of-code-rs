use macros::aoc_solver;

use anyhow::{ensure, Context, Result};

use crate::common::math_2d::Vec2;

static INPUT: &str = include_str!("input.txt");

#[aoc_solver(2025, 9, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    let points = parse_points(input)?;

    let mut max_size = 0;
    for p1 in points.iter().take(points.len() - 1) {
        for p2 in points.iter().skip(1) {
            let vec = *p2 - *p1;
            let size = (vec.x.abs() + 1) * (vec.y.abs() + 1);
            max_size = size.max(max_size);
        }
    }

    Ok(max_size.to_string())
}

fn parse_points(input: &str) -> Result<Vec<Vec2<i64>>> {
    let mut points = Vec::new();
    for line in input.lines() {
        let values = line
            .split(',')
            .map(|v| {
                v.parse::<i64>()
                    .with_context(|| format!("Failed to parse value {v}"))
            })
            .collect::<Result<Vec<i64>>>()?;

        ensure!(values.len() == 2, "Failed to parse line {}", line);

        points.push(Vec2::new(values[0], values[1]));
    }

    Ok(points)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve_part_1(include_str!("example.txt")).unwrap();
        assert_eq!(result, "50");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "4749838800");
    }
}
