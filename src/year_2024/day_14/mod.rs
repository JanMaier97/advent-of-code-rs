use std::{ops::{Add, Mul, Rem}, process::Output};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::MyResult;

mod part_1;

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
struct Point<T>{
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add<Vec2<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
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

struct Map {
    robots: Vec<Robot>,
    dim: Dimensions
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Robot {
    pos: Point<u64>,
    velocity: Vec2<i32>,
}

fn parse_input(input: &str) -> MyResult<Vec<Robot>> {
    input.lines()
        .map(|l| parse_robot(l))
        .collect::<Result<Vec<_>, _>>()
}

fn parse_robot(line: &str) -> MyResult<Robot> {
    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());
    
    let caputres = REGEX.captures(line).ok_or("Invalid line")?;
    let pos = Point {
        x: caputres[1].parse::<u64>()?,
        y: caputres[2].parse::<u64>()?,
    };
    let vel = Vec2 {
        x: caputres[3].parse::<i32>()?,
        y: caputres[4].parse::<i32>()?,
    };

    Ok(Robot {
        pos, velocity: vel
    })
}