//! # Fornjot Interop Types
//!
//! This is a temporary module, which contains the code from the deprecated
//! `fj-interop` crate. The types in here will, over time, move to more
//! appropriate places.

mod color;
mod tri_mesh;

pub mod ext;

pub use self::{
    color::Color,
    tri_mesh::{Index, MeshTriangle, TriMesh, vertices_to_indexed_vertices},
};
