use crate::handle::Handle;

use super::{curve::Curve, vertex::Vertex};

pub struct HalfEdge {
    pub curve: Handle<Curve>,
    pub start: Handle<Vertex>,
    pub is_internal: bool,
}
