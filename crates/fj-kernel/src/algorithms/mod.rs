//! Collection of algorithms that are used by the kernel
//!
//! Algorithmic code is collected in this module, to keep other modules focused
//! on their respective purpose.

mod approx;
mod contains;
mod reverse;
mod sweep;
mod transform;
mod triangulate;

pub mod cast_ray;
pub mod intersect;

pub use self::{
    approx::{CycleApprox, FaceApprox, InvalidTolerance, Tolerance},
    contains::Contains,
    reverse::reverse_face,
    sweep::sweep,
    transform::{transform_faces, TransformObject},
    triangulate::triangulate,
};
