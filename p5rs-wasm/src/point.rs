use crate::util::*;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub const fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn lerp(start: Point, end: Point, t: f64) -> Self {
        match t {
            0.0..=1.0 => start * (1. - t) + end * t,
            _ => {
                log(&"lerp: t was out of range 0..=1".to_string());
                log(&t);
                panic!();
            }
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
