use std::collections::HashSet;

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

#[aoc_solver(2025, 9, 2, INPUT)]
fn solve_part_2(input: &str) -> Result<String> {
    let points = parse_points(input)?;
    let orderd = get_points_in_order(&points);
    // let ps = points.iter().cloned().collect::<HashSet<_>>();
    // print_points(&ps);
    // let outline = generate_vertical_outline(&points);
    // let shape_points = get_points_in_outline(&outline);

    // print_points(&outline);
    // print_points(&shape_points);

    Ok("".to_string())
}

fn get_points_in_outline(outline: &HashSet<Vec2<i64>>) -> HashSet<Vec2<i64>> {
    let min_x = outline.iter().map(|p| p.x).min().unwrap();
    let max_x = outline.iter().map(|p| p.x).max().unwrap();
    let min_y = outline.iter().map(|p| p.y).min().unwrap();
    let max_y = outline.iter().map(|p| p.y).max().unwrap();

    let mut filled_outline = outline.clone();
    for y in min_y..=max_y {
        let mut current_pos_is_in_shpe = false;
        for x in min_x..=max_x {
            let point = Vec2::new(x, y);
            match (current_pos_is_in_shpe, outline.contains(&point)) {
                (false, false) => {
                    // do nothing
                }
                (false, true) => {
                    current_pos_is_in_shpe = true;
                }
                (true, false) => {
                    filled_outline.insert(point);
                }
                (true, true) => {
                    current_pos_is_in_shpe = false;
                }
            }
        }
    }

    filled_outline
}

fn generate_vertical_outline(points: &[Vec2<i64>]) -> HashSet<Vec2<i64>> {
    let ordered_points = get_points_in_order(points);

    // println!("ordered points: {:?}", ordered_points);
    let mut points_on_outline = HashSet::new();

    for (p1, p2) in ordered_points
        .iter()
        .take(ordered_points.len() - 1)
        .zip(ordered_points.iter().skip(1))
    {
        for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
            for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                if p1.y == p2.y {
                    continue;
                }
                points_on_outline.insert(Vec2::new(x, y));
            }
        }
    }
    // remove lines with an uneven amount of entries
    let min_y = points_on_outline.iter().map(|p| p.y).min().unwrap();
    let max_y = points_on_outline.iter().map(|p| p.y).max().unwrap();

    for y in min_y..max_y {
        let mut points_on_row = points_on_outline.iter().cloned().filter(|p| p.y == y).collect::<Vec<_>>();
        if points_on_row.len() % 2 == 0 {
            continue;
        }
        points_on_row.sort_by(|a, b| a.x.cmp(&b.x));
        for i in (1..points_on_row.len()).step_by(2) {
            points_on_outline.remove(&points_on_row[i]);
        }
    }



    points_on_outline
}

fn get_points_in_order(points: &[Vec2<i64>]) -> Vec<Vec2<i64>> {
    let mut ordered_points: Vec<Vec2<i64>> = Vec::with_capacity(points.len());
    ordered_points.push(points[0]);
    let mut points = points.into_iter().skip(0).collect::<HashSet<_>>();
    points.remove(&ordered_points[0]);
    loop {
        let last_point = ordered_points.last().unwrap();
        let next_point_x = points.iter().find(|p| p.x == last_point.x);
        let next_point_y = points.iter().find(|p| p.y == last_point.y);

        let Some(next_point) = next_point_y.or(next_point_x) else {
            break;
        };

        ordered_points.push(**next_point);
        points.remove(*next_point);
    }
    ordered_points.push(ordered_points[0]);
    ordered_points
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

fn print_points(points: &HashSet<Vec2<i64>>) {
    let min_x = points.iter().map(|p| p.x).min().unwrap() - 1;
    let max_x = points.iter().map(|p| p.x).max().unwrap() + 1;
    let min_y = points.iter().map(|p| p.y).min().unwrap() - 1;
    let max_y = points.iter().map(|p| p.y).max().unwrap() + 1;

    println!("=======================");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec2::new(x, y);
            if points.contains(&point) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("=======================");
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve_part_1(include_str!("example.txt")).unwrap();
        assert_eq!(result, "50");
    }

    #[test]
    fn solve_example_part_2() {
        let result = super::solve_part_2(include_str!("example.txt")).unwrap();
        assert_eq!(result, "24");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "4749838800");
    }
}
