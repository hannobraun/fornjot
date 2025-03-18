use crate::{geometry::TriMesh, math::Point, object::ToTriMesh};

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

impl ToTriMesh for Vertex {
    fn to_tri_mesh(&self) -> TriMesh {
        TriMesh::new()
    }
}
