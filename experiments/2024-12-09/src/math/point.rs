use std::ops;

use super::Vector;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point<const D: usize> {
    pub coords: Vector<D>,
}

impl<T> From<T> for Point<3>
where
    T: Into<Vector<3>>,
{
    fn from(coords: T) -> Self {
        Self {
            coords: coords.into(),
        }
    }
}

impl<T> ops::Add<T> for Point<3>
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
