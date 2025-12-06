use crate::{geometry::Vertex, store::Index};

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HalfEdge {
    pub vertices: [Index<Vertex>; 2],
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Face {
    pub boundary: [Index<HalfEdge>; 4],
}
