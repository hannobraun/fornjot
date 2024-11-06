use crate::math::Point;

use super::Operation;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex {
    pub point: Point,
}

impl<P> From<P> for Vertex
where
    P: Into<Point>,
{
    fn from(point: P) -> Self {
        Self {
            point: point.into(),
        }
    }
}

impl Operation for Vertex {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.push(*self);
    }

    fn triangles(&self, _: &mut Vec<Triangle>) {}
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl<V> From<[V; 3]> for Triangle
where
    V: Into<Vertex>,
{
    fn from(vertices: [V; 3]) -> Self {
        Self {
            vertices: vertices.map(Into::into),
        }
    }
}
