use std::ops;

use approx::AbsDiffEq;

use super::{Scalar, Vector};

/// An n-dimensional point
///
/// The dimensionality is defined by the const generic argument `D`.
///
/// # Implementation Note
///
/// The goal of this type is to eventually implement `Eq` and `Hash`, making it
/// easier to work with vectors. This is a work in progress.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<const D: usize>([Scalar; D]);

impl<const D: usize> Point<D> {
    /// Construct a `Point` at the origin of the coordinate system
    pub fn origin() -> Self {
        nalgebra::Point::<_, D>::origin().into()
    }

    /// Construct a `Point` from an array
    pub fn from_array(array: [f64; D]) -> Self {
        Self(array.map(Scalar::from_f64))
    }

    /// Construct a `Point` from an nalgebra vector
    pub fn from_na(point: nalgebra::Point<f64, D>) -> Self {
        Self(point.coords.data.0[0].map(Scalar::from_f64))
    }

    /// Convert the point into an nalgebra point
    pub fn to_na(&self) -> nalgebra::Point<f64, D> {
        self.0.map(Scalar::into_f64).into()
    }

    /// Convert to a 1-dimensional point
    pub fn to_t(&self) -> Point<1> {
        Point([self.0[0]])
    }

    /// Access a mutable reference to the point's z coordinate
    pub fn z_mut(&mut self) -> &mut Scalar {
        &mut self.0[2]
    }

    /// Access the point's coordinates as a vector
    pub fn coords(&self) -> Vector<D> {
        Vector::from(self.0)
    }
}

impl Point<1> {
    /// Access the curve point's t coordinate
    pub fn t(&self) -> Scalar {
        self.0[0]
    }
}

impl Point<2> {
    /// Access the point's x coordinate
    pub fn u(&self) -> Scalar {
        self.0[0]
    }

    /// Access the point's y coordinate
    pub fn v(&self) -> Scalar {
        self.0[1]
    }
}

impl Point<3> {
    /// Access the point's x coordinate
    pub fn x(&self) -> Scalar {
        self.0[0]
    }

    /// Access the point's y coordinate
    pub fn y(&self) -> Scalar {
        self.0[1]
    }

    /// Access the point's z coordinate
    pub fn z(&self) -> Scalar {
        self.0[2]
    }
}

impl<const D: usize> From<[Scalar; D]> for Point<D> {
    fn from(array: [Scalar; D]) -> Self {
        Self(array)
    }
}

impl<const D: usize> From<[f64; D]> for Point<D> {
    fn from(array: [f64; D]) -> Self {
        Self::from_array(array)
    }
}

impl<const D: usize> From<nalgebra::Point<f64, D>> for Point<D> {
    fn from(point: nalgebra::Point<f64, D>) -> Self {
        Self::from_na(point)
    }
}

impl<const D: usize> ops::Neg for Point<D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.to_na().neg().into()
    }
}

impl<const D: usize> ops::Add<Vector<D>> for Point<D> {
    type Output = Self;

    fn add(self, rhs: Vector<D>) -> Self::Output {
        self.to_na().add(rhs.to_na()).into()
    }
}

impl<const D: usize> ops::Sub<Point<D>> for Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Point<D>) -> Self::Output {
        self.to_na().sub(rhs.to_na()).into()
    }
}

impl<const D: usize> ops::Sub<Point<D>> for &Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Point<D>) -> Self::Output {
        self.to_na().sub(rhs.to_na()).into()
    }
}

impl<const D: usize> ops::Mul<f64> for Point<D> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.to_na().mul(rhs).into()
    }
}

impl<const D: usize> AbsDiffEq for Point<D> {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}
