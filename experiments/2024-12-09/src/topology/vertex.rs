use std::fmt;

use crate::{
    geometry::TriMesh,
    math::Point,
    object::{HandleAny, Object},
};

/// # A vertex
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex {
    pub point: Point<3>,
}

impl Vertex {
    pub fn new(point: impl Into<Point<3>>) -> Self {
        let point = point.into();
        Self { point }
    }
}

impl Object for Vertex {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vertex")
    }

    fn tri_mesh(&self) -> TriMesh {
        TriMesh::new()
    }

    fn children(&self) -> Vec<HandleAny> {
        Vec::new()
    }
}
