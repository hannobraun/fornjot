//! Sweeping objects along a path to create new objects

mod edge;
mod face;
mod sketch;

use fj_interop::mesh::Color;
use fj_math::Vector;

use super::Tolerance;

/// Sweep an object along a path to create another object
pub trait Sweep {
    /// The object that is created by sweeping the implementing object
    type Swept;

    /// Sweep the object along the given path
    fn sweep(
        self,
        path: impl Into<Path>,
        tolerance: Tolerance,
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
}

impl<T> From<T> for Path
where
    T: Into<Vector<3>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
