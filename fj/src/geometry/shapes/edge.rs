use nalgebra::vector;

use crate::{
    geometry::{operations::Sweep, shapes::Vertex},
    math::Vector,
};

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

    pub fn from_length(length: f32) -> Self {
        Sweep {
            shape: Vertex,
            path: vector![length],
        }
    }
}
