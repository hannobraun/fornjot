use std::{cmp::Ordering, ops};

use iter_fixed::IntoIteratorFixed;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub coords: Vector,
}

impl<T> From<T> for Point
where
    T: Into<Vector>,
{
    fn from(coords: T) -> Self {
        Self {
            coords: coords.into(),
        }
    }
}

impl<T> ops::Add<T> for Point
where
    T: Into<Vector>,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let other = other.into();
        let coords = self.coords + other;
        Self { coords }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vector {
    pub components: [Scalar; 3],
}

impl<S> From<[S; 3]> for Vector
where
    S: Into<Scalar>,
{
    fn from(components: [S; 3]) -> Self {
        Self {
            components: components.map(Into::into),
        }
    }
}

impl<T> ops::Add<T> for Vector
where
    T: Into<Vector>,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scalar {
    value: f64,
}

impl Scalar {
    /// # Create a new instance of [`Scalar`]
    ///
    /// ## Panics
    ///
    /// Panics, if `value` is NaN or infinite.
    pub fn new(value: f64) -> Self {
        if value.is_nan() {
            panic!("`Scalar` value must not be NaN");
        }
        if value.is_infinite() {
            panic!("`Scalar` value must not be infinite. Value: `{value}`");
        }

        Self { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Eq for Scalar {}

impl Ord for Scalar {
    fn cmp(&self, other: &Self) -> Ordering {
        let Some(ordering) = self.value.partial_cmp(&other.value) else {
            unreachable!(
                "Failed to compare `Scalar` values `{}` and `{}`",
                self.value, other.value
            );
        };

        ordering
    }
}

impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<f64> for Scalar {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl<T> ops::Add<T> for Scalar
where
    T: Into<Scalar>,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let value = self.value() + other.into().value();
        Self::new(value)
    }
}
