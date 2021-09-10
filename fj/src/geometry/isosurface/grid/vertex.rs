use nalgebra::Point;

use crate::util::vertices::AsPoint;

use super::Index;

/// A vertex in an isosurface extraction grid
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    /// The index that refers to this vertex
    pub index: Index,

    /// The position of the vertex
    pub point: Point<f32, 3>,

    /// The (signed) distance of the vertex to the closest surface point
    pub distance: f32,
}

impl AsPoint for Vertex {
    fn as_point(&self) -> Point<f32, 3> {
        self.point
    }
}
