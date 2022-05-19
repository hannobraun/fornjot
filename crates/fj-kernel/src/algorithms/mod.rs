//! Collection of algorithms that are used by the kernel
//!
//! Algorithmic code is collected in this module, to keep other modules focused
//! on their respective purpose.

mod approx;
mod sweep;
mod transform;
mod triangulation;

pub mod intersection;

pub use self::{
    approx::{CycleApprox, FaceApprox, Tolerance},
    sweep::sweep_shape,
    transform::transform_shape,
    triangulation::triangulate,
};
