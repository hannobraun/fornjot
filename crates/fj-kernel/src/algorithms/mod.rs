//! Collection of algorithms that are used by the kernel
//!
//! Algorithmic code is collected in this module, to keep other modules focused
//! on their respective purpose.

mod approx;
mod sweep;
mod transform;
mod triangulate;

pub mod intersection;

pub use self::{
    approx::{CycleApprox, FaceApprox, InvalidTolerance, Tolerance},
    sweep::sweep,
    transform::transform_shape,
    triangulate::triangulate,
};
