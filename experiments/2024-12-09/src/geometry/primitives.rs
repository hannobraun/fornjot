use std::fmt;

use crate::math::Point;

use super::{operation::AnyOp, Operation};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex {
    pub point: Point<3>,
}

impl<P> From<P> for Vertex
where
    P: Into<Point<3>>,
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

    fn children(&self) -> Vec<AnyOp> {
        Vec::new()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Triangle {
    pub vertices: [Point<3>; 3],
}

impl<P> From<[P; 3]> for Triangle
where
    P: Into<Point<3>>,
{
    fn from(vertices: [P; 3]) -> Self {
        Self {
            vertices: vertices.map(Into::into),
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

    fn children(&self) -> Vec<AnyOp> {
        Vec::new()
    }
}
