use std::fmt;

use crate::{
    math::Point,
    object::{HandleAny, Object},
};

use super::{MeshTriangle, TriMesh};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Triangle<const D: usize> {
    pub points: [Point<D>; 3],
}

impl<const D: usize> Triangle<D> {
    pub fn center(&self) -> Point<D> {
        let [a, b, c] = self.points;
        let coords = (a.coords + b.coords + c.coords) / 3.;
        Point { coords }
    }
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
            triangles: vec![MeshTriangle {
                inner: self.clone(),
            }],
        }
    }

    fn children(&self) -> Vec<HandleAny> {
        Vec::new()
    }
}
