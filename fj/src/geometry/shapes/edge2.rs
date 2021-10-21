use crate::geometry::{operations::Translate, shapes::Vertex};

/// An edge, defined by two vertices
#[derive(Debug)]
pub struct Edge2<const D: usize>(pub [Translate<Vertex, D>; 2]);

impl<T, const D: usize> From<[T; 2]> for Edge2<D>
where
    T: Into<Translate<Vertex, D>>,
{
    fn from(vertices: [T; 2]) -> Self {
        Self(vertices.map(|vertex| vertex.into()))
    }
}
