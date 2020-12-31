//! The trapezoidation tree
//!
//! During the trapezoidation process, the edges and vertices of the polygon are
//! inserted into the tree. The tree updates the metadata associated with each
//! trapezoid, allowing for later phases to process the trapezoids further.

mod id;

// The trapezoidation tree is implemented in multiple layers. Each layer knows
// about a specific aspect of the tree's structure and functionality and
// provides the layer above a solid base to build upon.
mod layer_a_connections;
mod layer_b_splitting;

pub use self::layer_b_splitting::Tree;
