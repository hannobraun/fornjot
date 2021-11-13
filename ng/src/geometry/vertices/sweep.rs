use std::vec;

use nalgebra::vector;

use crate::math::Point;

use super::Vertices;

impl Vertices for fj::Sweep {
    type Vertices = SweepVertices;

    fn vertices(&self) -> Self::Vertices {
        SweepVertices {
            original: self.shape.vertices(),
            length: self.length,
        }
    }
}

/// The vertices of a swept shape
///
/// See [`fj::Sweep`], specifically its implementation of [`Vertices`].
pub struct SweepVertices {
    /// The vertices of the original shape
    pub original: Vec<Point>,

    length: f32,
}

impl IntoIterator for SweepVertices {
    type Item = Point;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // TASK: Simplify implementation, once external code no longer relies on
        //       the order vertices.

        let mut vertices = Vec::new();

        for vertex in self.original {
            vertices.push(vertex);
            vertices.push(vertex + vector![0.0, 0.0, self.length]);
        }

        vertices.into_iter()
    }
}
