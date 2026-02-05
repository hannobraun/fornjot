use fj_math::{Point, Scalar, Triangle};

use crate::new::topology::Handle;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Vertex {
    pub point: Point<3>,
}

impl<S> From<[S; 3]> for Vertex
where
    S: Into<Scalar>,
{
    fn from(point: [S; 3]) -> Self {
        let point = Point::from(point.map(|s| s.into()));
        Self { point }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HalfEdge {
    pub boundary: [Handle<Vertex>; 2],
    pub approx: Vec<Point<3>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Face {
    pub boundary: Vec<Handle<HalfEdge>>,
    pub approx: Vec<Triangle<3>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Solid {
    pub boundary: Vec<Handle<Face>>,
}
