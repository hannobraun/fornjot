use std::ops;

use iter_fixed::IntoIteratorFixed;

use super::Scalar;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vector<const D: usize> {
    pub components: [Scalar; D],
}

impl<S, const D: usize> From<[S; D]> for Vector<D>
where
    S: Into<Scalar>,
{
    fn from(components: [S; D]) -> Self {
        Self {
            components: components.map(Into::into),
        }
    }
}

impl<T> ops::Add<T> for Vector<3>
where
    T: Into<Vector<3>>,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let other = other.into();

        let components = self
            .components
            .into_iter_fixed()
            .zip(other.components)
            .map(|(a, b)| a + b)
            .collect();

        Self { components }
    }
}
