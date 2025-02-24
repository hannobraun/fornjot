use crate::object::Handle;

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
