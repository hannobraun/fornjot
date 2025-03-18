use crate::{geometry::TriMesh, math::Point, object::Object};

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
    fn tri_mesh(&self) -> TriMesh {
        TriMesh::new()
    }
}
