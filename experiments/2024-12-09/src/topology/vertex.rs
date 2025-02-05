use crate::{
    geometry::{AnyOp, Operation, TriMesh},
    math::{Point, Vector},
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex {
    pub point: Point<3>,
}

impl Vertex {
    pub fn translate(self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            point: self.point + offset,
        }
    }
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
    type Output = Self;

    fn output(&self) -> &Self::Output {
        self
    }

    fn display(&self) -> &'static str {
        "Vertex"
    }

    fn tri_mesh(&self) -> TriMesh {
        TriMesh::new()
    }

    fn children(&self) -> Vec<AnyOp> {
        Vec::new()
    }
}
