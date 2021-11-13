use std::vec;

use nalgebra::vector;

use crate::math::Point;

use super::Vertices;

impl Vertices for fj::Sweep {
    type Vertices = SweepVertices;

    fn vertices(&self) -> Self::Vertices {
        let original = self.shape.vertices();
        let swept = original
            .iter()
            .map(|vertex| vertex + vector![0.0, 0.0, self.length])
            .collect();

        SweepVertices { original, swept }
    }
}

/// The vertices of a swept shape
///
/// See [`fj::Sweep`], specifically its implementation of [`Vertices`].
pub struct SweepVertices {
    /// The vertices of the original shape
    pub original: Vec<Point>,

    /// The new vertices created by sweeping the original shape
    pub swept: Vec<Point>,
}

impl IntoIterator for SweepVertices {
    type Item = Point;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // TASK: Simplify implementation, once external code no longer relies on
        //       the order vertices.

        let mut vertices = Vec::new();

        for (a, b) in self.original.into_iter().zip(self.swept) {
            vertices.push(a);
            vertices.push(b);
        }

        vertices.into_iter()
    }
}
