use nalgebra::Point;

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
