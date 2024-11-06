use crate::math::Point;

use super::Operation;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex {
    pub point: Point,
}

impl Operation for Vertex {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.push(*self);
    }

    fn triangles(&self, _: &mut Vec<Triangle>) {}
}

pub type Triangle = [Vertex; 3];
