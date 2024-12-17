use std::ops;

use super::Vector;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub coords: Vector<3>,
}

impl<T> From<T> for Point
where
    T: Into<Vector<3>>,
{
    fn from(coords: T) -> Self {
        Self {
            coords: coords.into(),
        }
    }
}

impl<T> ops::Add<T> for Point
where
    T: Into<Vector<3>>,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let other = other.into();
        let coords = self.coords + other;
        Self { coords }
    }
}
