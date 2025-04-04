use std::{cmp::Ordering, ops};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scalar {
    value: f64,
}

impl Scalar {
    pub fn new(value: f64) -> Self {
        if value.is_nan() {
            panic!("`Scalar` value must not be NaN");
        }
        if value.is_infinite() {
            panic!("`Scalar` value must not be infinite. Value: `{value}`");
        }

        Self { value }
    }

    pub fn zero() -> Self {
        Self::new(0.)
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn sqrt(self) -> Self {
        let value = self.value().sqrt();
        Self::new(value)
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

impl<S> ops::Add<S> for Scalar
where
    S: Into<Scalar>,
{
    type Output = Self;

    fn add(self, other: S) -> Self::Output {
        let value = self.value() + other.into().value();
        Self::new(value)
    }
}

impl<S> ops::Div<S> for Scalar
where
    S: Into<Scalar>,
{
    type Output = Self;

    fn div(self, other: S) -> Self::Output {
        let value = self.value() / other.into().value();
        Self::new(value)
    }
}

impl<S> ops::Mul<S> for Scalar
where
    S: Into<Scalar>,
{
    type Output = Self;

    fn mul(self, other: S) -> Self::Output {
        let value = self.value() * other.into().value();
        Self::new(value)
    }
}

impl ops::Neg for Scalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let value = -self.value();
        Self::new(value)
    }
}

impl<S> ops::Sub<S> for Scalar
where
    S: Into<Scalar>,
{
    type Output = Self;

    fn sub(self, other: S) -> Self::Output {
        let value = self.value() - other.into().value();
        Self::new(value)
    }
}
