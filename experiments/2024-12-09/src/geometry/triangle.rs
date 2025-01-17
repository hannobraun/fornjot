use crate::math::Point;

use super::{operation::AnyOp, Operation, TriMesh};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Triangle {
    pub points: [Point<3>; 3],
}

impl<P> From<[P; 3]> for Triangle
where
    P: Into<Point<3>>,
{
    fn from(points: [P; 3]) -> Self {
        Self {
            points: points.map(Into::into),
        }
    }
}

impl Operation for Triangle {
    fn label(&self) -> &'static str {
        "Triangle"
    }

    fn tri_mesh(&self) -> TriMesh {
        TriMesh {
            triangles: vec![self.clone()],
        }
    }

    fn children(&self) -> Vec<AnyOp> {
        Vec::new()
    }
}
