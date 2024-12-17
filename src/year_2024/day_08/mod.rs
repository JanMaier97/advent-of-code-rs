use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use anyhow::{anyhow, Result};

mod part_1;
mod part_2;

static INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Dimensions {
    width: usize,
    height: usize,
}

impl Dimensions {
    fn is_in_bounds(&self, point: Point2) -> bool {
        !(point.x < 0
            || point.y < 0
            || point.x as usize >= self.width
            || point.y as usize >= self.height)
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Frequency(char);

struct Map {
    dim: Dimensions,
    frequencies: HashMap<Frequency, HashSet<Point2>>,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn to(self, other: Point2) -> Vec2 {
        Vec2::new(other.x - self.x, other.y - self.y)
    }
}

impl Debug for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Add<Vec2> for Point2 {
    type Output = Point2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vec2> for Point2 {
    type Output = Point2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Point2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

fn parse_map(input: &str) -> Result<Map> {
    let height = input.lines().count();
    let width = input
        .lines()
        .map(|line| line.len())
        .max()
        .ok_or(anyhow!("Input is empyt"))?;

    let map = Map {
        frequencies: parse_freqency_nodes(input)?,
        dim: Dimensions { width, height },
    };

    Ok(map)
}

fn parse_freqency_nodes(input: &str) -> Result<HashMap<Frequency, HashSet<Point2>>> {
    let mut frequencies = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }

            let x = i32::try_from(x)?;
            let y = i32::try_from(y)?;

            frequencies
                .entry(Frequency(char))
                .or_insert(HashSet::new())
                .insert(Point2::new(x, y));
        }
    }

    Ok(frequencies)
}
