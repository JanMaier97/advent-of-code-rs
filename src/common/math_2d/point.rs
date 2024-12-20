use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Rem, Sub},
};

use super::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Debug> Debug for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?}, {:?})", self.x, self.y))
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
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

impl<T: AddAssign> AddAssign<Vec2<T>> for Point<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub<Vec2<T>> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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
