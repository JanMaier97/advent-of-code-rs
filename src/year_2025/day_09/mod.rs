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
    let mut outline = get_points_on_outline(&points);
    let points_in_outline = get_points_in_outline(&points, &outline);


    println!("done");

    // print_points(&outline);
    // print_points(&points_in_outline);
    // outline.extend(points_in_outline.iter());
    // print_points(&outline);
    
    // let mut max_size = 0;
    // for p1 in points.iter().take(points.len() - 1) {
    //     for p2 in points.iter().skip(1) {
    //         let vec = *p2 - *p1;
    //         let size = (vec.x.abs() + 1) * (vec.y.abs() + 1);
    //         max_size = size.max(max_size);
    //     }
    // }

    Ok("".to_string())
}

fn get_points_in_outline(corners: &[Vec2<i64>], outline: &HashSet<Vec2<i64>>) -> HashSet<Vec2<i64>> {
    let (e1, e2, dir) = get_edge_and_inward_direction(corners);
    let start_point = e1 + (get_direction_normal(e1, e2) * 2) + dir;

    let mut points_in_shape = HashSet::from_iter([start_point]);
    let mut points_to_visit = points_in_shape.clone();
    let mut next_points = HashSet::new();

    loop {
        for point in points_to_visit.iter() {
            for neightbour in get_neighbours(*point) {
                if points_in_shape.contains(&neightbour) || outline.contains(&neightbour) {
                    continue;
                }
                next_points.insert(neightbour);
                points_in_shape.insert(neightbour);
            }
        }

        if next_points.is_empty() {
            break;
        }

        points_to_visit.clear();
        points_to_visit.extend(next_points.drain());
    }

    points_in_shape
}

fn get_neighbours(point: Vec2<i64>) -> [Vec2<i64>; 8] {
    [
        point + Vec2::new(-1, -1),
        point + Vec2::new(00, -1),
        point + Vec2::new(01, -1),
        point + Vec2::new(-1, 00),
        point + Vec2::new(01, 00),
        point + Vec2::new(-1, 01),
        point + Vec2::new(00, 01),
        point + Vec2::new(01, 01),
    ]
}

fn get_edge_and_inward_direction(corners: &[Vec2<i64>]) -> (Vec2<i64>,Vec2<i64>,Vec2<i64>) {
    let points_in_order = get_points_in_order(corners);

    let p1_iter = points_in_order.iter().take(points_in_order.len() - 3);
    let p2_iter = points_in_order.iter().skip(1).take(points_in_order.len() - 2);
    let p3_iter = points_in_order.iter().skip(2).take(points_in_order.len() - 1);
    let p4_iter = points_in_order.iter().skip(3);

    for (((p1, p2), p3), p4) in p1_iter.zip(p2_iter).zip(p3_iter).zip(p4_iter) {
        assert!(p2.x == p3.x || p2.y == p3.y);
        let dir1 = get_direction_normal(*p2, *p1);
        let dir2 = get_direction_normal(*p3, *p4);

        if dir1 != dir2 {
            continue;
        }

        return (*p2, *p3, dir1);
    };

    panic!("Failed to find quadrupple that faces inward...");
}

fn get_points_on_outline(points: &[Vec2<i64>]) -> HashSet<Vec2<i64>> {
    let ordered = get_points_in_order(&points);

    let mut outline = HashSet::new();
    for (p1, p2) in ordered.iter().take(ordered.len() - 1).zip(ordered.iter().skip(1)) {
        let edge_points = get_points_on_edge(*p1, *p2);
        outline.extend(edge_points.iter());
    }


    outline
}

fn get_points_on_edge(p1: Vec2<i64>, p2: Vec2<i64>) -> Vec<Vec2<i64>> {
    assert!(p1.x == p2.x || p1.y == p1.y);

    let mut points = vec![p1];
    let dir = get_direction_normal(p1, p2);
    let mut current_pos = p1;
    loop {
        current_pos = current_pos + dir;
        points.push(current_pos);
        if current_pos == p2 {
            break;
        }
    }

    points
}


fn get_direction_normal(from: Vec2<i64>, to: Vec2<i64>) -> Vec2<i64> {
    let dir = to - from;

    let x = dir.x / dir.x.abs().max(1);
    let y = dir.y / dir.y.abs().max(1);

    return Vec2::new(x, y);
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
    use crate::{common::math_2d::Vec2, year_2025::day_09::{get_points_on_outline, parse_points, print_points}};

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn solve_example_part_1() {
        let result = super::solve_part_1(EXAMPLE).unwrap();
        assert_eq!(result, "50");
    }

    #[test]
    fn solve_example_part_2() {
        let result = super::solve_part_2(EXAMPLE).unwrap();
        assert_eq!(result, "24");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "4749838800");
    }
}
