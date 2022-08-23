//! Collection of algorithms that are used by the kernel
//!
//! Algorithmic code is collected in this module, to keep other modules focused
//! on their respective purpose.

mod approx;
mod reverse;
mod transform;
mod triangulate;

pub mod intersect;
pub mod sweep;

pub use self::{
    approx::{CycleApprox, FaceApprox, InvalidTolerance, Tolerance},
    reverse::reverse_face,
    transform::{transform_faces, TransformObject},
    triangulate::triangulate,
};
