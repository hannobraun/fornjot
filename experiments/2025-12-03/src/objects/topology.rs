use std::ops;

use crate::{
    objects::geometry::{Triangle, Vertex},
    store::{Index, Store},
};

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HalfEdge {
    pub vertices: [Index<Vertex>; 2],
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Face {
    pub boundary: [Index<HalfEdge>; 4],
    pub triangles: [Index<Triangle>; 2],
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
