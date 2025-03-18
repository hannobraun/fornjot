use crate::{
    geometry::TriMesh,
    object::{Handle, HandleAny, Object},
};

use super::vertex::Vertex;

pub struct HalfEdge {
    pub start: Handle<Vertex>,
    pub is_internal: bool,
}

impl Object for HalfEdge {
    fn tri_mesh(&self) -> TriMesh {
        TriMesh::new()
    }

    fn children(&self) -> Vec<HandleAny> {
        vec![self.start.to_any()]
    }
}
