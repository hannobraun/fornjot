//! Sweeping objects along a path to create new objects

mod curve;
mod edge;
mod face;
mod sketch;
mod vertex;

use fj_math::Vector;

use crate::stores::Stores;

/// Sweep an object along a path to create another object
pub trait Sweep {
    /// The object that is created by sweeping the implementing object
    type Swept;

    /// Sweep the object along the given path
    fn sweep(self, path: impl Into<Vector<3>>, store: &Stores) -> Self::Swept;
}
