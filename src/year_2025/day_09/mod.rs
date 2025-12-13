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
    let max_x =  get_max_x(&points);
    let outline = get_points_on_outline(&points);

    // print_points(&outline);

    let mut max_size = 0;
    for p1 in points.iter().take(points.len() - 1) {
        for p2 in points.iter().skip(1) {
            if !rect_inside_shape(*p1, *p2, &outline, max_x) {
                continue;
            }
            let vec = *p2 - *p1;
            let size = (vec.x.abs() + 1) * (vec.y.abs() + 1);
            max_size = size.max(max_size);
        }
    }

    Ok(max_size.to_string())
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

fn rect_inside_shape(p1: Vec2<i64>, p2: Vec2<i64>, outline: &HashSet<Vec2<i64>>, max_x: i64) -> bool {
    let points = vec![p1, Vec2::new(p1.x, p2.y), p2, Vec2::new(p2.x, p1.y), p1];

    if !point_inside_shape(points[1], outline, max_x) || !point_inside_shape(points[3], outline, max_x) {
        return false;
    }
    // println!("rect points: {points:?}");
    for (e1, e2) in points.iter().take(points.len()-1).zip(points.iter().skip(1)) {
        assert!(e1.x == e2.x || e1.y == e2.y, "points {e1:?} and {e2:?} are not on the same line\npoints: {points:?}");
        let dir = get_direction_normal(*e1, *e2);
        let mut current_pos = *e1;
        loop {
            if !point_inside_shape(current_pos, outline, max_x) {
                return false;
            }
            current_pos = current_pos + dir;
            if current_pos == *e2 {
                break;
            }
        }
    }

    true
}

fn get_max_x(points: &[Vec2<i64>]) -> i64 {
    points.iter().map(|p| p.x).max().unwrap() + 1
}


fn point_inside_shape(point: Vec2<i64>, outline: &HashSet<Vec2<i64>>, max_x: i64) -> bool {
   if outline.contains(&point) {
       return true;
   }
   
   // println!("Checking point {point:?}");
   // println!("Max x: {max_x}");

   let mut intersections = 0;
   let mut intersection_started = false;
   let mut on_edge = false;
   for x in point.x..=max_x {
       let next_point = Vec2::new(x, point.y);
       // println!("Probing cast at {next_point:?}");

       let on_outline = outline.contains(&next_point);
       if on_outline && intersection_started {
           on_edge = true;
       }

       if on_outline && !intersection_started {
           // println!("Found intersection");
           intersections += 1;
           intersection_started = true;
       }

       if !on_outline && intersection_started {
           // println!("reset intersection flag");
           intersection_started = false;
           if on_edge { 
               // println!("reset on_edge flag, adding intersection");
               intersections += 1;
               on_edge = false;
           }
       }
   }

   // println!("Found {intersections} intersections for point {point:?}");

   intersections % 2 == 1
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
    use crate::{common::math_2d::Vec2, year_2025::day_09::{get_max_x, get_points_on_outline, parse_points, point_inside_shape, print_points, rect_inside_shape}};

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

    #[test]
    fn check_rect_in_shape() {
        let input = EXAMPLE;
        let points = parse_points(input).unwrap();
        let max_x =  get_max_x(&points);
        let outline = get_points_on_outline(&points);

        assert!(point_inside_shape(Vec2::new(7, 1), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(7, 2), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(7, 3), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(8, 3), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(9, 3), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(10, 3), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(11, 3), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(11, 2), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(11, 1), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(10, 1), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(9, 1), &outline, max_x));
        assert!(point_inside_shape(Vec2::new(8, 1), &outline, max_x));

        assert!(!point_inside_shape(Vec2::new(0, 1), &outline, max_x));
        assert!(!point_inside_shape(Vec2::new(6, 1), &outline, max_x));
        assert!(!point_inside_shape(Vec2::new(0, 4), &outline, max_x));
    }

}
