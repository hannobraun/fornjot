use crate::handle::Handle;

use super::{curve::Curve, vertex::Vertex};

#[derive(Debug)]
pub struct HalfEdge {
    pub curve: Handle<Curve>,
    pub start: Handle<Vertex>,
    pub is_internal: bool,
}
