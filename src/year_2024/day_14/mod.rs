use std::ops::{Add, Mul, Rem};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::MyResult;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Dimensions {
    height: u64,
    width: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T: Mul<Output = T> + Clone> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add<Vec2<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Rem<Output = T> + Clone> std::ops::Rem<T> for Point<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self {
            x: self.x % rhs.clone(),
            y: self.y % rhs,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Robot {
    pos: Point<u64>,
    velocity: Vec2<i32>,
}

fn parse_input(input: &str) -> MyResult<Vec<Robot>> {
    input
        .lines()
        .map(parse_robot)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_robot(line: &str) -> MyResult<Robot> {
    static REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

    let caputres = REGEX.captures(line).ok_or("Invalid line")?;
    let pos = Point {
        x: caputres[1].parse::<u64>()?,
        y: caputres[2].parse::<u64>()?,
    };
    let vel = Vec2 {
        x: caputres[3].parse::<i32>()?,
        y: caputres[4].parse::<i32>()?,
    };

    Ok(Robot { pos, velocity: vel })
}

fn move_robot(robot: Robot, dim: Dimensions, times: u64) -> MyResult<Point<u64>> {
    let normal_vel = normalize_velocity(robot.velocity, dim);
    let next_pos = robot.pos + normal_vel * times;

    Ok(Point {
        x: next_pos.x % dim.width,
        y: next_pos.y % dim.height,
    })
}

fn normalize_velocity(velocity: Vec2<i32>, dim: Dimensions) -> Vec2<u64> {
    Vec2 {
        x: normalize_value(velocity.x, dim.width),
        y: normalize_value(velocity.y, dim.height),
    }
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
            velocity: Vec2 { x: 2, y: -3 },
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
