use std::fmt;

use crate::{
    math::Point,
    object::{HandleAny, Object},
};

use super::TriMesh;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Triangle<const D: usize> {
    pub points: [Point<D>; 3],
}

impl<P, const D: usize> From<[P; 3]> for Triangle<D>
where
    P: Into<Point<D>>,
{
    fn from(points: [P; 3]) -> Self {
        Self {
            points: points.map(Into::into),
        }
    }
}

impl Object for Triangle<3> {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Triangle")
    }

    fn tri_mesh(&self) -> TriMesh {
        TriMesh {
            triangles: vec![self.clone()],
        }
    }

    fn children(&self) -> Vec<HandleAny> {
        Vec::new()
    }
}
