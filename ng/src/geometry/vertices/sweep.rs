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

impl IntoIterator for SweepVertices {
    type Item = Point;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // TASK: Simplify implementation, once external code no longer relies on
        //       the order vertices.

        let mut vertices = Vec::new();

        for vertex in self.0.shape.vertices() {
            vertices.push(vertex);
            vertices.push(vertex + vector![0.0, 0.0, self.0.length]);
        }

        vertices.into_iter()
    }
}
