use std::{fmt, ops};

use super::{
    coordinates::{Uv, Xyz, T},
    Scalar,
};

/// An n-dimensional vector
///
/// The dimensionality is defined by the const generic argument `D`.
///
/// # Implementation Note
///
/// The goal of this type is to eventually implement `Eq` and `Hash`, making it
/// easier to work with vectors. This is a work in progress.
#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Vector<const D: usize>(pub [Scalar; D]);

impl<const D: usize> Vector<D> {
    /// Construct a `Vector` from an array
    pub fn from_array(array: [f64; D]) -> Self {
        Self(array.map(Scalar::from_f64))
    }

    /// Construct a `Vector` from an nalgebra vector
    pub fn from_na(vector: nalgebra::SVector<f64, D>) -> Self {
        Self::from_array(vector.into())
    }

    /// Convert the vector into an nalgebra vector
    pub fn to_na(&self) -> nalgebra::SVector<f64, D> {
        self.0.map(Scalar::into_f64).into()
    }

    /// Convert to a 1-dimensional vector
    pub fn to_t(&self) -> Vector<1> {
        Vector([self.0[0]])
    }

    /// Compute the magnitude of the vector
    pub fn magnitude(&self) -> Scalar {
        self.to_na().magnitude().into()
    }

    /// Compute a normalized version of the vector
    pub fn normalize(&self) -> Self {
        self.to_na().normalize().into()
    }

    /// Compute the dot product with another vector
    pub fn dot(&self, other: &Self) -> Scalar {
        self.to_na().dot(&other.to_na()).into()
    }

    /// Compute the cross product with another vector
    pub fn cross(&self, other: &Self) -> Self {
        self.to_na().cross(&other.to_na()).into()
    }

    /// Access an iterator over the vector's components
    pub fn components(&self) -> [Scalar; D] {
        self.0
    }
}

impl Vector<2> {
    /// Extend a 2-dimensional vector into a 3-dimensional one
    pub fn to_xyz(&self, z: Scalar) -> Vector<3> {
        Vector::from([self.u, self.v, z])
    }
}

impl Vector<3> {
    /// Construct a new vector from this vector's x and y components
    pub fn xy(&self) -> Vector<2> {
        Vector::from([self.x, self.y])
    }
}

impl ops::Deref for Vector<1> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.0.as_ptr() as *const Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &*ptr }
    }
}

impl ops::Deref for Vector<2> {
    type Target = Uv;

    fn deref(&self) -> &Self::Target {
        let ptr = self.0.as_ptr() as *const Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &*ptr }
    }
}

impl ops::Deref for Vector<3> {
    type Target = Xyz;

    fn deref(&self) -> &Self::Target {
        let ptr = self.0.as_ptr() as *const Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &*ptr }
    }
}

impl ops::DerefMut for Vector<1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.0.as_mut_ptr() as *mut Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &mut *ptr }
    }
}

impl ops::DerefMut for Vector<2> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.0.as_mut_ptr() as *mut Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &mut *ptr }
    }
}

impl ops::DerefMut for Vector<3> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.0.as_mut_ptr() as *mut Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &mut *ptr }
    }
}

impl<const D: usize> From<[Scalar; D]> for Vector<D> {
    fn from(array: [Scalar; D]) -> Self {
        Self(array)
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

impl<const D: usize> From<Vector<D>> for [f32; D] {
    fn from(vector: Vector<D>) -> Self {
        vector.0.map(|scalar| scalar.into_f32())
    }
}

impl<const D: usize> From<Vector<D>> for [f64; D] {
    fn from(vector: Vector<D>) -> Self {
        vector.0.map(|scalar| scalar.into_f64())
    }
}

impl<const D: usize> From<Vector<D>> for nalgebra::SVector<f64, D> {
    fn from(vector: Vector<D>) -> Self {
        vector.to_na()
    }
}

impl<const D: usize> ops::Add<Self> for Vector<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.to_na().add(rhs.to_na()).into()
    }
}

impl<const D: usize> ops::Mul<Scalar> for Vector<D> {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.to_na().mul(rhs.into_f64()).into()
    }
}

impl<const D: usize> ops::Div<Scalar> for Vector<D> {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        self.to_na().div(rhs.into_f64()).into()
    }
}

impl<const D: usize> fmt::Debug for Vector<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const D: usize> approx::AbsDiffEq for Vector<D> {
    type Epsilon = <Scalar as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}
