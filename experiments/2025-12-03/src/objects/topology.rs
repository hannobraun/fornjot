use fj_math::{Point, Triangle};

use crate::store::Index;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Vertex {
    pub point: Point<3>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HalfEdge {
    pub boundary: [Index<Vertex>; 2],
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Face {
    pub boundary: Vec<Index<HalfEdge>>,
    pub approx: Vec<Triangle<3>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Solid {
    pub boundary: Vec<Index<Face>>,
}
