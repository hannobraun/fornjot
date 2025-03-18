use crate::{
    geometry::TriMesh,
    object::{Handle, Object},
};

use super::vertex::Vertex;

pub struct HalfEdge {
    pub start: Handle<Vertex>,
    pub is_internal: bool,
}

impl Object for HalfEdge {
    fn to_tri_mesh(&self) -> TriMesh {
        TriMesh::new()
    }
}
