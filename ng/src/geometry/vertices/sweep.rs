use nalgebra::vector;

use crate::math::Point;

use super::Vertices;

impl Vertices for fj::Sweep {
    type Vertices = Vec<Point>;

    fn vertices(&self) -> Self::Vertices {
        let mut vertices = Vec::new();

        for vertex in self.shape.vertices() {
            vertices.push(vertex);
            vertices.push(vertex + vector![0.0, 0.0, self.length]);
        }

        vertices
    }
}
