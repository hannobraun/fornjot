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

impl Operation for Vertex {
    fn label(&self) -> &'static str {
        "Vertex"
    }

    fn tri_mesh(&self) -> TriMesh {
        TriMesh::new()
    }

    fn children(&self) -> Vec<AnyOp> {
        Vec::new()
    }
}
