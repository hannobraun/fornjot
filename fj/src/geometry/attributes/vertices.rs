use crate::geometry::{operations::Translate, shapes::Vertex};

/// Implemented by shapes that can return the vertices that make them up
///
/// Since the vertices of a shape are going to have a position in space, `D`
/// defines the dimension of those vertices' positions.
pub trait Vertices<const D: usize> {
    fn vertices(&self) -> Vec<Translate<Vertex, D>>;
}

impl<const D: usize> Vertices<D> for Translate<Vertex, D> {
    fn vertices(&self) -> Vec<Translate<Vertex, D>> {
        vec![*self]
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use crate::{geometry::shapes::Vertex, prelude::*};

    use super::Vertices as _;

    #[test]
    fn test_vertices_for_translate_vertices() {
        let vertex_1d = Vertex.translate(vector![2.0]);
        let vertex_2d = Vertex.translate(vector![2.0, 3.0]);
        let vertex_3d = Vertex.translate(vector![2.0, 3.0, 4.0]);

        assert_eq!(vertex_1d.vertices(), [vertex_1d]);
        assert_eq!(vertex_2d.vertices(), [vertex_2d]);
        assert_eq!(vertex_3d.vertices(), [vertex_3d]);
    }
}
