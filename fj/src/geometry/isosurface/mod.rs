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

pub mod grid;
pub mod qef;
pub mod to_mesh;

pub use self::{grid::Grid, to_mesh::to_mesh};
