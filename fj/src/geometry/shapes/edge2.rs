use crate::geometry::{operations::Translate, shapes::Vertex};

/// An edge, defined by two vertices
pub struct Edge2<const D: usize>(pub [Translate<Vertex, D>; 2]);
