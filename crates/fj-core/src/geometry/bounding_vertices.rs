use crate::{
    objects::Vertex,
    storage::{Handle, HandleWrapper},
};

/// The bounding vertices of an edge
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct BoundingVertices {
    /// The bounding vertices
    pub inner: [HandleWrapper<Vertex>; 2],
}

impl BoundingVertices {
    /// Normalize the bounding vertices
    ///
    /// Returns a new instance of this struct, which has the vertices in a
    /// defined order. This can be used to compare bounding vertices while
    /// disregarding their order.
    pub fn normalize(mut self) -> Self {
        self.inner.sort();
        self
    }
}

impl From<[Handle<Vertex>; 2]> for BoundingVertices {
    fn from(vertices: [Handle<Vertex>; 2]) -> Self {
        Self {
            inner: vertices.map(Into::into),
        }
    }
}
