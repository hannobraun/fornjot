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

impl<V, const D: usize> ops::Add<V> for Vector<D>
where
    V: Into<Vector<D>>,
{
    type Output = Self;

    fn add(self, other: V) -> Self::Output {
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

impl<S, const D: usize> ops::Mul<S> for Vector<D>
where
    S: Into<Scalar>,
{
    type Output = Self;

    fn mul(self, scalar: S) -> Self::Output {
        let scalar = scalar.into();

        let components = self
            .components
            .into_iter_fixed()
            .map(|v| v * scalar)
            .collect();

        Self { components }
    }
}
