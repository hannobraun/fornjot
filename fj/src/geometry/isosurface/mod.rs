//! Simple isosurface extraction algorithm
//!
//! This algorithm is based on ideas from [Dual Contouring of Hermite Data], and
//! might be similar to the Surface Nets algorithm.
//!
//! [Manifold Dual Contouring] might also become relevant, as that's one of the
//! papers I'm looking at right now for improvements.
//!
//! [Dual Contouring of Hermite Data]: https://www.cse.wustl.edu/~taoju/research/dualContour.pdf
//! [Manifold Dual Contouring]: https://people.engr.tamu.edu/schaefer/research/dualsimp_tvcg.pdf

pub mod edge;
pub mod grid;
pub mod grid_descriptor;
pub mod grid_index;
pub mod to_mesh;

pub use self::{
    edge::{Edge, Value},
    grid::Grid,
    grid_descriptor::GridDescriptor,
    grid_index::GridIndex,
    to_mesh::to_mesh,
};
