use std::ops;

use approx::AbsDiffEq;

/// An n-dimensional vector
///
/// The dimensionality is defined by the const generic argument `D`.
///
/// # Implementation Note
///
/// The goal of this type is to eventually implement `Eq` and `Hash`, making it
/// easier to work with vectors. This is a work in progress.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<const D: usize>([f64; D]);

impl<const D: usize> Vector<D> {
    /// Construct a `Vector` from an array
    ///
    /// # Implementation Note
    ///
    /// All vector construction functions should call this method internally. At
    /// some point, this will become the place where validate the floating point
    /// numbers before constructing the vector instance.
    pub fn from_array(array: [f64; D]) -> Self {
        Self(array)
    }

    /// Construct a `Vector` from an nalgebra vector
    pub fn from_na(vector: nalgebra::SVector<f64, D>) -> Self {
        Self::from_array(vector.into())
    }

    /// Convert the vector into an nalgebra vector
    pub fn to_na(&self) -> nalgebra::SVector<f64, D> {
        self.0.into()
    }

    /// Access the vector's x coordinate
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    /// Access the vector's y coordinate
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    /// Construct a new vector from this vector's x and y components
    pub fn xy(&self) -> Vector<2> {
        Vector::from([self.x(), self.y()])
    }

    /// Compute the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        self.to_na().magnitude()
    }

    /// Compute a normalized version of the vector
    pub fn normalize(&self) -> Self {
        self.to_na().normalize().into()
    }

    /// Compute the dot product with another vector
    pub fn dot(&self, other: &Self) -> f64 {
        self.to_na().dot(&other.to_na())
    }
}

impl Vector<1> {
    /// Access the curve vector's t coordinate
    pub fn t(&self) -> f64 {
        self.0[0]
    }
}

impl Vector<2> {
    /// Access the surface vector's u coordinate
    pub fn u(&self) -> f64 {
        self.0[0]
    }

    /// Access the surface vector's v coordinate
    pub fn v(&self) -> f64 {
        self.0[1]
    }

    /// Extend a 2-dimensional vector into a 3-dimensional one
    pub fn extend(&self, scalar: f64) -> Vector<3> {
        Vector::from([self.u(), self.v(), scalar])
    }
}

impl<const D: usize> From<[f64; D]> for Vector<D> {
    fn from(array: [f64; D]) -> Self {
        Self::from_array(array)
    }
}

impl<const D: usize> From<nalgebra::SVector<f64, D>> for Vector<D> {
    fn from(vector: nalgebra::SVector<f64, D>) -> Self {
        Self::from_na(vector)
    }
}

impl<const D: usize> ops::Add<Self> for Vector<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.to_na().add(rhs.to_na()).into()
    }
}

impl<const D: usize> ops::Mul<f64> for Vector<D> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.to_na().mul(rhs).into()
    }
}

impl<const D: usize> AbsDiffEq for Vector<D> {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}
