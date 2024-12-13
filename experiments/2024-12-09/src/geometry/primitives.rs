use std::fmt;

use crate::math::Point;

use super::{operation::HandleAny, Operation};

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

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [x, y, z] = self.point.coords.components.map(|s| s.value());
        write!(f, "vertex {x:.2}, {y:.2}, {z:.2}")
    }
}

impl Operation for Vertex {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.push(*self);
    }

    fn triangles(&self, _: &mut Vec<Triangle>) {}

    fn children(&self) -> Vec<HandleAny> {
        Vec::new()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl From<[&Vertex; 3]> for Triangle {
    fn from(vertices: [&Vertex; 3]) -> Self {
        Self {
            vertices: vertices.map(|vertex| *vertex),
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "triangle")
    }
}

impl Operation for Triangle {
    fn vertices(&self, _: &mut Vec<Vertex>) {}

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        triangles.push(self.clone())
    }

    fn children(&self) -> Vec<HandleAny> {
        self.vertices
            .iter()
            .map(|vertex| HandleAny::new(*vertex))
            .collect()
    }
}
