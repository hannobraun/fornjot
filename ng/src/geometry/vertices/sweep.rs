use std::vec;

use nalgebra::vector;

use crate::math::Point;

use super::Vertices;

impl Vertices for fj::Sweep {
    type Vertices = SweepVertices;

    fn vertices(&self) -> Self::Vertices {
        SweepVertices(self.clone())
    }
}

/// The vertices of a swept shape
///
/// See [`fj::Sweep`], specifically its implementation of [`Vertices`].
pub struct SweepVertices(fj::Sweep);

impl SweepVertices {
    /// Compute pairs of original and swept vertices
    ///
    /// Returns each vertex of the original shape, together with the vertex that
    /// was created by sweeping it.
    pub fn vertex_pairs(&self) -> Vec<[Point; 2]> {
        let mut pairs = Vec::new();

        for vertex in self.0.shape.vertices() {
            let a = vertex;
            let b = vertex + vector![0.0, 0.0, self.0.length];

            pairs.push([a, b]);
        }

        pairs
    }
}

impl IntoIterator for SweepVertices {
    type Item = Point;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vertices = Vec::new();

        for [a, b] in self.vertex_pairs() {
            vertices.push(a);
            vertices.push(b);
        }

        vertices.into_iter()
    }
}
