use crate::geometry::{operations::Translate, shapes::Vertex};

/// An edge, defined by two vertices
pub type Edge2<const D: usize> = [Translate<Vertex, D>; 2];
