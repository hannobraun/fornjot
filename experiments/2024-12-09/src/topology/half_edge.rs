use std::fmt;

use crate::{
    geometry::TriMesh,
    object::{Handle, HandleAny, Object},
};

use super::vertex::Vertex;

pub struct HalfEdge {
    start: Handle<Vertex>,
}

impl HalfEdge {
    pub fn new(start: Handle<Vertex>) -> Self {
        Self { start }
    }

    pub fn start(&self) -> &Handle<Vertex> {
        &self.start
    }
}

impl Object for HalfEdge {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HalfEdge")
    }

    fn tri_mesh(&self) -> TriMesh {
        TriMesh::new()
    }

    fn children(&self) -> Vec<HandleAny> {
        vec![self.start.to_any()]
    }
}
