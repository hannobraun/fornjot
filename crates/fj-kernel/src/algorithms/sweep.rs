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
        path: impl Into<Vector<3>>,
        tolerance: Tolerance,
        color: Color,
    ) -> Self::Swept;
}
