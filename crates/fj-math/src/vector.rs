use std::{fmt, ops};

use super::{
    coordinates::{Uv, Xyz, T},
    Scalar,
};

/// An n-dimensional vector
///
/// The dimensionality of the vector is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Vector<const D: usize> {
    /// The vector components
    pub components: [Scalar; D],
}

impl<const D: usize> Vector<D> {
    /// Construct a `Vector` from `f64` components
    ///
    /// # Panics
    ///
    /// Panics, if the components can not be converted to [`Scalar`]. See
    /// [`Scalar::from_f64`], which this method uses internally.
    pub fn from_components_f64(components: [f64; D]) -> Self {
        Self {
            components: components.map(Scalar::from_f64),
        }
    }

    /// Construct a `Vector` from an nalgebra vector
    pub fn from_na(vector: nalgebra::SVector<f64, D>) -> Self {
        Self::from_components_f64(vector.into())
    }

    /// Convert the vector into an nalgebra vector
    pub fn to_na(self) -> nalgebra::SVector<f64, D> {
        self.components.map(Scalar::into_f64).into()
    }

    /// Convert to a 1-dimensional vector
    pub fn to_t(self) -> Vector<1> {
        Vector {
            components: [self.components[0]],
        }
    }

    /// Convert the vector into a 2-dimensional vector
    ///
    /// If the vector is 0-, or 1-dimensional, the missing components will be
    /// initialized to zero.
    ///
    /// If the vector has higher dimensionality than two, the superfluous
    /// components will be discarded.
    pub fn to_uv(self) -> Vector<2> {
        let zero = Scalar::ZERO;

        let components = match self.components.as_slice() {
            [] => [zero, zero],
            &[t] => [t, zero],
            &[u, v, ..] => [u, v],
        };

        Vector { components }
    }

    /// Convert the vector into a 3-dimensional vector
    ///
    /// If the vector is 0-, 1-, or 2-dimensional, the missing components will
    /// be initialized to zero.
    ///
    /// If the vector has higher dimensionality than three, the superfluous
    /// components will be discarded.
    pub fn to_xyz(self) -> Vector<3> {
        let zero = Scalar::ZERO;

        let components = match self.components.as_slice() {
            [] => [zero, zero, zero],
            &[t] => [t, zero, zero],
            &[u, v] => [u, v, zero],
            &[x, y, z, ..] => [x, y, z],
        };

        Vector { components }
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

    /// Compute the scalar projection of this vector onto another
    pub fn scalar_projection_onto(&self, other: &Self) -> Scalar {
        if other.magnitude() == Scalar::ZERO {
            return Scalar::ZERO;
        }

        self.dot(&other.normalize())
    }
}

impl Vector<1> {
    /// Construct a `Vector` that represents the t-axis
    pub fn unit_t() -> Self {
        Self::from([1.])
    }
}

impl Vector<2> {
    /// Construct a `Vector` that represents the u-axis
    pub fn unit_u() -> Self {
        Self::from([1., 0.])
    }

    /// Construct a `Vector` that represents the v-axis
    pub fn unit_v() -> Self {
        Self::from([0., 1.])
    }

    /// Compute the 2D cross product with another vector
    pub fn cross2d(&self, other: &Self) -> Scalar {
        (self.u * other.v) - (self.v * other.u)
    }

    /// Determine whether this vector is between two other vectors
    pub fn is_between(&self, others: [impl Into<Self>; 2]) -> bool {
        let [a, b] = others.map(Into::into);
        a.cross2d(self) * b.cross2d(self) < Scalar::ZERO
    }
}

impl Vector<3> {
    /// Construct a `Vector` that represents the x-axis
    pub fn unit_x() -> Self {
        Self::from([1., 0., 0.])
    }

    /// Construct a `Vector` that represents the y-axis
    pub fn unit_y() -> Self {
        Self::from([0., 1., 0.])
    }

    /// Construct a `Vector` that represents the z-axis
    pub fn unit_z() -> Self {
        Self::from([0., 0., 1.])
    }

    /// Compute the cross product with another vector
    pub fn cross(&self, other: &Self) -> Self {
        self.to_na().cross(&other.to_na()).into()
    }

    /// Construct a new vector from this vector's x and y components
    pub fn xy(&self) -> Vector<2> {
        Vector::from([self.x, self.y])
    }
}

impl ops::Deref for Vector<1> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.components.as_ptr() as *const Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &*ptr }
    }
}

impl ops::Deref for Vector<2> {
    type Target = Uv;

    fn deref(&self) -> &Self::Target {
        let ptr = self.components.as_ptr() as *const Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &*ptr }
    }
}

impl ops::Deref for Vector<3> {
    type Target = Xyz;

    fn deref(&self) -> &Self::Target {
        let ptr = self.components.as_ptr() as *const Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &*ptr }
    }
}

impl ops::DerefMut for Vector<1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.components.as_mut_ptr() as *mut Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &mut *ptr }
    }
}

impl ops::DerefMut for Vector<2> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.components.as_mut_ptr() as *mut Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &mut *ptr }
    }
}

impl ops::DerefMut for Vector<3> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.components.as_mut_ptr() as *mut Self::Target;

        // This is sound. We've created this pointer from a valid instance, that
        // has the same size and layout as the target.
        unsafe { &mut *ptr }
    }
}

impl<const D: usize> Default for Vector<D> {
    fn default() -> Self {
        let components = [Scalar::default(); D];
        Self { components }
    }
}

impl<const D: usize> From<[Scalar; D]> for Vector<D> {
    fn from(components: [Scalar; D]) -> Self {
        Self { components }
    }
}

impl<const D: usize> From<[f64; D]> for Vector<D> {
    fn from(components: [f64; D]) -> Self {
        Self::from_components_f64(components)
    }
}

impl<const D: usize> From<nalgebra::SVector<f64, D>> for Vector<D> {
    fn from(vector: nalgebra::SVector<f64, D>) -> Self {
        Self::from_na(vector)
    }
}

impl<const D: usize> From<Vector<D>> for [f32; D] {
    fn from(vector: Vector<D>) -> Self {
        vector.components.map(|scalar| scalar.into_f32())
    }
}

impl<const D: usize> From<Vector<D>> for [f64; D] {
    fn from(vector: Vector<D>) -> Self {
        vector.components.map(|scalar| scalar.into_f64())
    }
}

impl<const D: usize> From<Vector<D>> for [Scalar; D] {
    fn from(vector: Vector<D>) -> Self {
        vector.components
    }
}

impl<const D: usize> From<Vector<D>> for nalgebra::SVector<f64, D> {
    fn from(vector: Vector<D>) -> Self {
        vector.to_na()
    }
}

impl<const D: usize> ops::Neg for Vector<D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.to_na().neg().into()
    }
}

impl<V, const D: usize> ops::Add<V> for Vector<D>
where
    V: Into<Self>,
{
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        self.to_na().add(rhs.into().to_na()).into()
    }
}

impl<V, const D: usize> ops::Sub<V> for Vector<D>
where
    V: Into<Self>,
{
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        self.to_na().sub(rhs.into().to_na()).into()
    }
}

impl<S, const D: usize> ops::Mul<S> for Vector<D>
where
    S: Into<Scalar>,
{
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        self.to_na().mul(rhs.into().into_f64()).into()
    }
}

impl<S, const D: usize> ops::MulAssign<S> for Vector<D>
where
    S: Into<Scalar>,
{
    fn mul_assign(&mut self, rhs: S) {
        *self = *self * rhs;
    }
}

impl<S, const D: usize> ops::Div<S> for Vector<D>
where
    S: Into<Scalar>,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        self.to_na().div(rhs.into().into_f64()).into()
    }
}

impl<const D: usize> fmt::Debug for Vector<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.components.fmt(f)
    }
}

impl<const D: usize> approx::AbsDiffEq for Vector<D> {
    type Epsilon = <Scalar as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        Scalar::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.components.abs_diff_eq(&other.components, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Scalar, Vector};

    #[test]
    fn to_uv() {
        let d0: [f64; 0] = [];
        assert_eq!(Vector::from(d0).to_uv(), Vector::from([0., 0.]));
        assert_eq!(Vector::from([1.]).to_uv(), Vector::from([1., 0.]));
        assert_eq!(Vector::from([1., 2.]).to_uv(), Vector::from([1., 2.]));
        assert_eq!(Vector::from([1., 2., 3.]).to_uv(), Vector::from([1., 2.]),);
    }

    #[test]
    fn to_xyz() {
        let d0: [f64; 0] = [];
        assert_eq!(Vector::from(d0).to_xyz(), Vector::from([0., 0., 0.]));
        assert_eq!(Vector::from([1.]).to_xyz(), Vector::from([1., 0., 0.]));
        assert_eq!(Vector::from([1., 2.]).to_xyz(), Vector::from([1., 2., 0.]));
        assert_eq!(
            Vector::from([1., 2., 3.]).to_xyz(),
            Vector::from([1., 2., 3.]),
        );
    }

    #[test]
    fn scalar_projection_onto() {
        let v = Vector::from([1., 2., 3.]);

        let x = Vector::unit_x() * 3.;
        let y = Vector::unit_y() * 2.;
        let z = Vector::unit_z() * 1.;

        assert_eq!(v.scalar_projection_onto(&x), Scalar::from(1.));
        assert_eq!(v.scalar_projection_onto(&y), Scalar::from(2.));
        assert_eq!(v.scalar_projection_onto(&z), Scalar::from(3.));

        // Zero-length vectors should be handled as well.
        assert_eq!(
            Vector::unit_x()
                .scalar_projection_onto(&Vector::from([0., 0., 0.])),
            Scalar::ZERO
        );
    }

    #[test]
    fn is_between() {
        let v = Vector::from([1., 1.]);

        assert!(v.is_between([[1., 0.], [0., 1.]]));
        assert!(!v.is_between([[1., 0.], [0., -1.]]));
        assert!(!v.is_between([[-1., 0.], [0., 1.]]));
    }
}
