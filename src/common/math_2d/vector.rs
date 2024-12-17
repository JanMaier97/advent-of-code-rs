use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vec2<i32> {
    pub const UP: Vec2<i32> = Self { x: 0, y: -1 };
    pub const DOWN: Vec2<i32> = Self { x: 0, y: 1 };
    pub const LEFT: Vec2<i32> = Self { x: -1, y: 0 };
    pub const RIGHT: Vec2<i32> = Self { x: 1, y: 0 };
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
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
