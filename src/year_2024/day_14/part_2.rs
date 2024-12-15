use std::{collections::HashSet, io::stdin};

use macros::aoc_solver;

use crate::MyResult;

use super::{move_robot, parse_input, Dimensions, Point, Robot};

#[aoc_solver(2024, 14, 2, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let dim = Dimensions {
        height: 103,
        width: 101,
    };

    let robots = parse_input(input)?;
    find_christmas_tree(&robots, dim)?;

    Ok(0)
}

fn find_christmas_tree(robots: &[Robot], dim: Dimensions) -> MyResult<()> {
    for times in 0.. {
        if times % 1000 == 0 {
            println!("iteration {}", times);
        }

        let positions = get_robot_positions(robots, dim, times)?;
        if !is_likely_a_christmas_tree(&positions) {
            continue;
        }

        print_positions(&positions, dim);
        println!("Iteration {}", times);

        stdin().read_line(&mut String::new()).unwrap();
    }

    Ok(())
}

fn get_robot_positions(
    robots: &[Robot],
    dim: Dimensions,
    times: u64,
) -> MyResult<HashSet<Point<u64>>> {
    robots
        .iter()
        .map(|r| move_robot(*r, dim, times))
        .collect::<Result<HashSet<_>, _>>()
}

fn print_positions(pos: &HashSet<Point<u64>>, dim: Dimensions) {
    for y in 0..dim.height {
        let line = (0..dim.width)
            .map(|x| {
                if pos.contains(&Point { x, y }) {
                    "#"
                } else {
                    "."
                }
            })
            .collect::<String>();
        println!("{}", line);
    }
}

fn is_likely_a_christmas_tree(positions: &HashSet<Point<u64>>) -> bool {
    let vertical_center_count = positions
        .iter()
        .filter(|pos| pos.y > 43 && pos.y < 77)
        .count() as f64;

    let horizontal_center_count = positions
        .iter()
        .filter(|pos| pos.x > 30 && pos.x < 70)
        .count() as f64;

    let limit = positions.len() as f64 * 0.7;

    limit <= vertical_center_count || limit <= horizontal_center_count
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_14::{parse_input, Dimensions};

    use super::{get_robot_positions, print_positions};

    #[test]
    fn test() {
        let dim = Dimensions {
            width: 11,
            height: 7,
        };
        let r = parse_input(include_str!("example.txt")).unwrap();
        let pos = get_robot_positions(&r, dim, 100).unwrap();
        print_positions(&pos, dim);

        assert_eq!(0, 1);
    }
}
