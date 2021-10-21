use std::fmt;

use crate::geometry::{operations::Translate, shapes::Vertex};

// TASK: Replace `Edge` with this.
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

impl<const D: usize> fmt::Display for Edge2<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.0[0].display(), self.0[1].display())
    }
}
