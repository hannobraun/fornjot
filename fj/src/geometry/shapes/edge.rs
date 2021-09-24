use nalgebra::vector;

use crate::{
    geometry::{operations::Sweep, shapes::Vertex},
    math::Vector,
};

pub type Edge = Sweep<Vertex, Vector<1>>;

impl Edge {
    pub fn from_length(length: f32) -> Self {
        Sweep {
            shape: Vertex,
            path: vector![length],
        }
    }
}
