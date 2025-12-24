use std::ops;

use fj_math::{Point, Triangle};

use crate::store::{Index, Store};

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
    pub triangles: Vec<Triangle<3>>,
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Solid {
    pub boundary: Vec<Index<Face>>,
}

#[derive(Default)]
pub struct Faces {
    inner: Store<Face>,
}

impl Faces {
    pub fn push(&mut self, face: Face) -> Index<Face> {
        self.inner.push(face)
    }
}

impl ops::Index<Index<Face>> for Faces {
    type Output = Face;

    fn index(&self, index: Index<Face>) -> &Self::Output {
        &self.inner[index]
    }
}
