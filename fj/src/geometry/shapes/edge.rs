use nalgebra::vector;

use crate::{
    geometry::{operations::Sweep, shapes::Vertex},
    math::Vector,
};

/// A 1-dimensional edge
///
/// Defined as a sweep of a 0-dimensional `Vertex` over a straight path of a
/// given length.
pub type Edge = Sweep<Vertex, Vector<1>>;

impl Edge {
    /// Create a new `Edge`
    ///
    /// The length is initially set to `1.0`.
    pub fn new() -> Self {
        Sweep {
            shape: Vertex,
            path: vector![1.0],
        }
    }

    /// Update length
    ///
    /// Returns a copy of `self`, with the length replaced with `length`.
    pub fn with_length(mut self, length: f32) -> Self {
        self.path.x = length;
        self
    }
}
