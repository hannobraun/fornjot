use std::{fmt, ops};

use super::{
    Scalar, Vector,
    coordinates::{T, Uv, Xyz},
};

/// An n-dimensional point
///
/// The dimensionality of the point is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Point<const D: usize> {
    /// The coordinates of the point
    pub coords: Vector<D>,
}

impl<const D: usize> Point<D> {
    /// Construct a `Point` at the origin of the coordinate system
    pub fn origin() -> Self {
        nalgebra::Point::<_, D>::origin().into()
    }

    /// Construct a `Point` from an array
    pub fn from_array(array: [f64; D]) -> Self {
        Self {
            coords: array.map(Scalar::from_f64).into(),
        }
    }

    /// Construct a `Point` from an nalgebra vector
    pub fn from_na(point: nalgebra::Point<f64, D>) -> Self {
        Self {
            coords: point.coords.into(),
        }
    }

    /// Convert the point into an nalgebra point
    pub fn to_na(self) -> nalgebra::Point<f64, D> {
        nalgebra::Point {
            coords: self.coords.into(),
        }
    }

    /// Convert to a 3-dimensional point
    ///
    /// See [`Vector::to_xyz`] for details. This method follows the same rules.
    pub fn to_xyz(self) -> Point<3> {
        Point {
            coords: self.coords.to_xyz(),
        }
    }

    /// Gives the distance between two points.
    pub fn distance_to(&self, other: &Self) -> Scalar {
        (self.coords - other.coords).magnitude()
    }
}

impl ops::Deref for Point<1> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.coords.deref()
    }
}

impl ops::Deref for Point<2> {
    type Target = Uv;

    fn deref(&self) -> &Self::Target {
        self.coords.deref()
    }
}

impl ops::Deref for Point<3> {
    type Target = Xyz;

    fn deref(&self) -> &Self::Target {
        self.coords.deref()
    }
}

impl ops::DerefMut for Point<1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.coords.deref_mut()
    }
}

impl ops::DerefMut for Point<2> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.coords.deref_mut()
    }
}

impl ops::DerefMut for Point<3> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.coords.deref_mut()
    }
}

impl<const D: usize> From<[Scalar; D]> for Point<D> {
    fn from(array: [Scalar; D]) -> Self {
        Self {
            coords: array.into(),
        }
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

impl<const D: usize> From<Point<D>> for [f32; D] {
    fn from(point: Point<D>) -> Self {
        point.coords.into()
    }
}

impl<const D: usize> From<Point<D>> for [f64; D] {
    fn from(point: Point<D>) -> Self {
        point.coords.into()
    }
}

impl<const D: usize> From<Point<D>> for [Scalar; D] {
    fn from(point: Point<D>) -> Self {
        point.coords.into()
    }
}

impl<const D: usize> ops::Neg for Point<D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.to_na().neg().into()
    }
}

impl<V, const D: usize> ops::Add<V> for Point<D>
where
    V: Into<Vector<D>>,
{
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        self.to_na().add(rhs.into().to_na()).into()
    }
}

impl<V, const D: usize> ops::AddAssign<V> for Point<D>
where
    V: Into<Vector<D>>,
{
    fn add_assign(&mut self, rhs: V) {
        *self = *self + rhs;
    }
}

impl<V, const D: usize> ops::Sub<V> for Point<D>
where
    V: Into<Vector<D>>,
{
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        self.to_na().sub(rhs.into().to_na()).into()
    }
}

impl<const D: usize> ops::Sub<Self> for Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.to_na().sub(rhs.to_na()).into()
    }
}

impl<const D: usize> ops::Sub<Point<D>> for &Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Point<D>) -> Self::Output {
        self.to_na().sub(rhs.to_na()).into()
    }
}

impl<V, const D: usize> ops::SubAssign<V> for Point<D>
where
    V: Into<Vector<D>>,
{
    fn sub_assign(&mut self, v: V) {
        *self = *self - v.into();
    }
}

impl<const D: usize> fmt::Debug for Point<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.coords.fmt(f)
    }
}

impl<const D: usize> approx::AbsDiffEq for Point<D> {
    type Epsilon = <Vector<D> as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        Scalar::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.coords.abs_diff_eq(&other.coords, epsilon)
    }
}
