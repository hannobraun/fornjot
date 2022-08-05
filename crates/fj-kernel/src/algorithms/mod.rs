//! Collection of algorithms that are used by the kernel
//!
//! Algorithmic code is collected in this module, to keep other modules focused
//! on their respective purpose.

mod approx;
mod reverse;
mod sweep;
mod transform;
mod triangulate;

pub mod intersection;
pub mod ray_cast;

pub use self::{
    approx::{CycleApprox, FaceApprox, InvalidTolerance, Tolerance},
    reverse::reverse_face,
    sweep::sweep,
    transform::{transform_faces, TransformObject},
    triangulate::triangulate,
};
