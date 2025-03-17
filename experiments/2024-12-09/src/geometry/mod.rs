//! # Various geometry tools
//! 
//! These are distinct from the core b-rep representation, for which this module
//! is a dependency.

mod sketch;
mod surface;
mod tri_mesh;
mod triangle;

pub use self::{
    sketch::Sketch,
    surface::SurfaceGeometry,
    tri_mesh::{MeshTriangle, TriMesh},
    triangle::Triangle,
};
