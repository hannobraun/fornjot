//! Approximation of objects

pub mod curve;
pub mod cycle;
pub mod edge;
pub mod face;
pub mod tolerance;

pub use self::{
    cycle::CycleApprox,
    face::FaceApprox,
    tolerance::{InvalidTolerance, Tolerance},
};

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
