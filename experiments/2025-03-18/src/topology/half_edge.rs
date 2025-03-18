use crate::{
    geometry::{ToTriMesh, TriMesh},
    object::Handle,
};

use super::vertex::Vertex;

pub struct HalfEdge {
    pub start: Handle<Vertex>,
    pub is_internal: bool,
}

impl ToTriMesh for HalfEdge {
    fn to_tri_mesh(&self) -> TriMesh {
        TriMesh::new()
    }
}
