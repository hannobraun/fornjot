//! # Tools for creating approximations
//!
//! Fornjot can be viewed as a hybrid b-rep/mesh-based kernel. Topology is
//! represented using typical b-rep primitives, but geometry is approximated
//! with polylines and triangle meshes.
//!
//! Topological primitives and geometrical approximations exist side by side and
//! approximations are built up together with the topological primitives. The
//! tools provided by this module help doing that.
//!
//! This module is intended for internal use, as well as more advanced users of
//! Fornjot. It is typically required to implement operations that create and
//! modify shapes. More basic users would just use operations that others have
//! implemented, never coming into contact with this module.

mod axis;
mod face;
mod half_edge;
mod point;

pub use self::{
    axis::ApproxAxis, face::face_approx, half_edge::ApproxHalfEdge,
    point::ApproxPoint,
};
