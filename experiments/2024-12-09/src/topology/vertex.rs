use std::fmt;

use crate::{
    geometry::{AnyOp, Operation, TriMesh},
    math::Point,
};

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
    fn triangles(&self, _: &mut TriMesh) {}

    fn children(&self) -> Vec<AnyOp> {
        Vec::new()
    }
}
