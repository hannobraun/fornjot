//! Sweeping objects along a path to create new objects

mod edge;
mod face;
mod sketch;
mod vertex;

use fj_interop::mesh::Color;
use fj_math::{Scalar, Vector};

use super::approx::Tolerance;

/// Sweep an object along a path to create another object
pub trait Sweep {
    /// The object that is created by sweeping the implementing object
    type Swept;

    /// Sweep the object along the given path
    fn sweep(
        self,
        path: impl Into<Path>,
        tolerance: impl Into<Tolerance>,
        color: Color,
    ) -> Self::Swept;
}

/// A path to be used with [`Sweep`]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Path(Vector<3>);

impl Path {
    /// Return the vector that defines this path
    pub fn inner(&self) -> Vector<3> {
        self.0
    }

    /// Indicate whether the path is in the negative direction
    pub fn is_negative_direction(&self) -> bool {
        self.0.dot(&Vector::from([0., 0., 1.])) < Scalar::ZERO
    }
}

impl<T> From<T> for Path
where
    T: Into<Vector<3>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
