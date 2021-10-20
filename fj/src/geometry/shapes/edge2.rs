use crate::geometry::{operations::Translate, shapes::Vertex};

/// An edge, defines by two vertices
pub type Edge2<const D: usize> = (Translate<Vertex, D>, Translate<Vertex, D>);
