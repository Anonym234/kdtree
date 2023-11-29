use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use super::KDPoint;

#[derive(Debug, Clone, Copy)]
pub struct F64(f64);

impl From<f64> for F64 {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for F64 {}

impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        f64::partial_cmp(&self.0, &other.0)
    }
}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f64::total_cmp(&self.0, &other.0)
    }
}

impl Add<F64> for F64 {
    type Output = Self;

    fn add(self, rhs: F64) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl Sub<F64> for F64 {
    type Output = Self;

    fn sub(self, rhs: F64) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}

impl Mul<F64> for F64 {
    type Output = Self;

    fn mul(self, rhs: F64) -> Self::Output {
        (self.0 * rhs.0).into()
    }
}

impl Into<f64> for F64 {
    fn into(self) -> f64 {
        self.0
    }
}

impl Display for F64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Point2D {
    fn from(value: (f64, f64)) -> Self {
        let (x, y) = value;
        Self { x, y }
    }
}

impl From<[f64; 2]> for Point2D {
    fn from(value: [f64; 2]) -> Self {
        let [x, y] = value;
        Self { x, y }
    }
}

impl KDPoint for Point2D {
    type Key = F64;
    type Distance = F64;

    fn kdkey(&self, dimension: usize) -> Self::Key {
        match dimension % 2 {
            0 => self.x,
            1 => self.y,
            _ => unreachable!(),
        }
        .into()
    }

    fn distance(lhs: &Self, rhs: &Self) -> Self::Distance {
        let xdiff = lhs.x - rhs.x;
        let ydiff = lhs.y - rhs.y;
        (xdiff * xdiff + ydiff * ydiff).into()
    }

    fn key_distance(lhs: &Self::Key, rhs: &Self::Key) -> Self::Distance {
        let dist = *lhs - *rhs;
        dist * dist
    }
}

#[derive(Debug, Clone)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<I: Into<T>, T> From<(I, I, I)> for Point3D<T> {
    fn from(value: (I, I, I)) -> Self {
        let (x, y, z) = value;
        [x, y, z].into()
    }
}

impl<I: Into<T>, T> From<[I; 3]> for Point3D<T> {
    fn from(value: [I; 3]) -> Self {
        let [x, y, z] = value.map(Into::into);
        Self { x, y, z }
    }
}

impl<T: Ord + Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T>> KDPoint
    for Point3D<T>
{
    type Key = T;

    type Distance = T;

    fn kdkey(&self, dimension: usize) -> Self::Key {
        match dimension % 3 {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => unreachable!(),
        }
    }

    fn distance(lhs: &Self, rhs: &Self) -> Self::Distance {
        let xdiff = lhs.x - rhs.x;
        let ydiff = lhs.y - rhs.y;
        let zdiff = lhs.z - rhs.z;

        (xdiff * xdiff + ydiff * ydiff + zdiff * zdiff).into()
    }

    fn key_distance(lhs: &Self::Key, rhs: &Self::Key) -> Self::Distance {
        let dist = *lhs - *rhs;
        dist * dist
    }
}
