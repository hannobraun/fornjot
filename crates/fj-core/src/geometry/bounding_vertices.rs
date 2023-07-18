use crate::{
    objects::Vertex,
    storage::{Handle, HandleWrapper},
};

/// The bounding vertices of an edge
#[derive(Eq, PartialEq)]
pub struct BoundingVertices {
    /// The bounding vertices
    pub inner: [HandleWrapper<Vertex>; 2],
}

impl From<[Handle<Vertex>; 2]> for BoundingVertices {
    fn from(vertices: [Handle<Vertex>; 2]) -> Self {
        Self {
            inner: vertices.map(Into::into),
        }
    }
}
