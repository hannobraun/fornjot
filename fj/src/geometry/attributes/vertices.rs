use crate::geometry::{operations::Translate, shapes::Vertex};

/// Implemented by shapes that can return the vertices that make them up
///
/// Since the vertices of a shape are going to have a position in space, `D`
/// defines the dimension of those vertices' positions.
pub trait Vertices<const D: usize> {
    fn vertices(&self) -> Vec<Translate<Vertex, D>>;
}
