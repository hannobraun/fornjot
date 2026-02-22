//! # Fornjot Interop Types
//!
//! This is a temporary module, which contains the code from the deprecated
//! `fj-interop` crate. The types in here will, over time, move to more
//! appropriate places.

mod approx;
mod color;
mod tolerance;
mod tri_mesh;

pub mod ext;

pub use self::{
    approx::CircleApproxParams,
    color::Color,
    tolerance::{InvalidTolerance, Tolerance},
    tri_mesh::{Index, MeshTriangle, TriMesh, vertices_to_indexed_vertices},
};
