use crate::handle::Handle;

use super::vertex::Vertex;

pub struct HalfEdge {
    pub start: Handle<Vertex>,
    pub is_internal: bool,
}
