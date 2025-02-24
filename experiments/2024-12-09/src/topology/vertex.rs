use std::fmt;

use crate::{
    geometry::TriMesh,
    math::{Point, Vector},
    object::{HandleAny, Object},
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex {
    pub point: Point<3>,
}

impl Vertex {
    pub fn new(point: impl Into<Point<3>>) -> Self {
        let point = point.into();
        Self { point }
    }

    pub fn translate(self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            point: self.point + offset,
        }
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
