use once_cell::sync::Lazy;
use regex::Regex;

use crate::common::math_2d::{Dimensions, Point, Vec2};

use anyhow::{anyhow, Result};

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Robot {
    pos: Point<u64>,
    velocity: Vec2<i32>,
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    input
        .lines()
        .map(parse_robot)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_robot(line: &str) -> Result<Robot> {
    static REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

    let caputres = REGEX.captures(line).ok_or(anyhow!("Invalid line"))?;
    let pos = Point::new(caputres[1].parse::<u64>()?, caputres[2].parse::<u64>()?);
    let vel = Vec2::new(caputres[3].parse::<i32>()?, caputres[4].parse::<i32>()?);

    Ok(Robot { pos, velocity: vel })
}

fn move_robot(robot: Robot, dim: Dimensions, times: u64) -> Result<Point<u64>> {
    let normal_vel = normalize_velocity(robot.velocity, dim);
    let next_pos = robot.pos + normal_vel * times;

    Ok(Point::new(next_pos.x % dim.width, next_pos.y % dim.height))
}

fn normalize_velocity(velocity: Vec2<i32>, dim: Dimensions) -> Vec2<u64> {
    Vec2::new(
        normalize_value(velocity.x, dim.width),
        normalize_value(velocity.y, dim.height),
    )
}

fn normalize_value(value: i32, identity: u64) -> u64 {
    if value < 0 {
        let abs: u64 = value.unsigned_abs().into();
        let times = abs / identity;
        let rem = abs % identity;

        if abs >= identity {
            println!(
                "normalized {} to {}",
                value,
                identity * times + identity - rem
            );
        }

        identity * times + identity - rem
    } else {
        value as u64
    }
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_14::{move_robot, Dimensions, Point, Robot, Vec2};

    #[test]
    fn robots_move_with_wrapping() {
        let robot = Robot {
            pos: Point { x: 2, y: 4 },
            velocity: Vec2::new(2, -3),
        };
        let dim = Dimensions {
            height: 7,
            width: 11,
        };

        let pos = move_robot(robot, dim, 1).unwrap();
        assert_eq!(pos, Point { x: 4, y: 1 });

        let pos = move_robot(robot, dim, 2).unwrap();
        assert_eq!(pos, Point { x: 6, y: 5 });

        let pos = move_robot(robot, dim, 3).unwrap();
        assert_eq!(pos, Point { x: 8, y: 2 });

        let pos = move_robot(robot, dim, 4).unwrap();
        assert_eq!(pos, Point { x: 10, y: 6 });

        let pos = move_robot(robot, dim, 5).unwrap();
        assert_eq!(pos, Point { x: 1, y: 3 });
    }
}
