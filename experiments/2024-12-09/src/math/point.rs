use std::ops;

use super::Vector;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point<const D: usize> {
    pub coords: Vector<D>,
}

impl<V, const D: usize> From<V> for Point<D>
where
    V: Into<Vector<D>>,
{
    fn from(coords: V) -> Self {
        Self {
            coords: coords.into(),
        }
    }
}

impl<V, const D: usize> ops::Add<V> for Point<D>
where
    V: Into<Vector<D>>,
{
    type Output = Self;

    fn add(self, other: V) -> Self::Output {
        let other = other.into();
        let coords = self.coords + other;
        Self { coords }
    }
}
