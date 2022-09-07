//! Approximation of objects

pub mod curve;
pub mod cycle;
pub mod edge;
pub mod face;
pub mod tolerance;

use fj_math::Point;

pub use self::tolerance::{InvalidTolerance, Tolerance};

/// Approximate an object
pub trait Approx {
    /// The approximation of the object
    type Approximation;

    /// Approximate the object
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual object.
    fn approx(self, tolerance: Tolerance) -> Self::Approximation;
}

/// A point from an approximation, with local and global forms
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ApproxPoint<const D: usize> {
    /// The local form of the point
    pub local_form: Point<D>,

    /// The global form of the points
    pub global_form: Point<3>,
}
