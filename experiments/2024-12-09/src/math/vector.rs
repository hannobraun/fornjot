use std::ops;

use iter_fixed::IntoIteratorFixed;

use super::Scalar;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vector<const D: usize> {
    pub components: [Scalar; D],
}

impl<const D: usize> Vector<D> {
    pub fn magnitude(&self) -> Scalar {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(&self, other: &Self) -> Scalar {
        self.components
            .into_iter()
            .zip(other.components)
            .map(|(a, b)| a * b)
            .reduce(|a, b| a + b)
            .unwrap_or(Scalar::zero())
    }
}

impl Vector<3> {
    pub fn cross(self, other: Self) -> Self {
        let [ax, ay, az] = self.components;
        let [bx, by, bz] = other.components;

        Self {
            components: [
                ay * bz - az * by,
                az * bx - ax * bz,
                ax * by - ay * bx,
            ],
        }
    }
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

impl<S, const D: usize> ops::Div<S> for Vector<D>
where
    S: Into<Scalar>,
{
    type Output = Self;

    fn div(self, scalar: S) -> Self::Output {
        let scalar = scalar.into();
        let components = self.components.map(|component| component / scalar);
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

impl<const D: usize> ops::Neg for Vector<D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl<V, const D: usize> ops::Sub<V> for Vector<D>
where
    V: Into<Vector<D>>,
{
    type Output = Self;

    fn sub(self, other: V) -> Self::Output {
        let other = other.into();

        let components = self
            .components
            .into_iter_fixed()
            .zip(other.components)
            .map(|(a, b)| a - b)
            .collect();

        Self { components }
    }
}
