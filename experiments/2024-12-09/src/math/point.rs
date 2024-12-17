use std::ops;

use super::Vector;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point<const D: usize> {
    pub coords: Vector<D>,
}

impl<T, const D: usize> From<T> for Point<D>
where
    T: Into<Vector<D>>,
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
