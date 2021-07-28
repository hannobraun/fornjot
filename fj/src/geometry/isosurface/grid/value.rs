use nalgebra::Point;

use super::Index;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub index: Index,
    pub point: Point<f32, 3>,
    pub distance: f32,
}
