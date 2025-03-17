use std::fmt;

use crate::{
    geometry::TriMesh,
    object::{Handle, HandleAny, Object},
};

use super::vertex::Vertex;

/// # A half-edge
///
/// Half-edges bound faces. A half-edge only contains the vertex where it
/// starts. The end vertex is implicit (it is the start vertex of the next
/// half-edge in the same face). By doing it like this, each half-edge "owns"
/// its vertex, which simplifies the object graph, making it easier to change.
///
/// Since a face only has a single boundary, that boundary needs to touch itself
/// to connect the outside of the face with any holes on the inside. The
/// half-edges that touch other half-edges are marked as "internal".
pub struct HalfEdge {
    pub start: Handle<Vertex>,
    pub is_internal: bool,
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
