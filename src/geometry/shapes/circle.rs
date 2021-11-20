use nalgebra::point;

use crate::{
    geometry::{
        bounding_volume::Aabb,
        edges::{Edge, Edges},
        faces::{triangulate, Faces},
        Shape,
    },
    math::Point,
};

impl Shape for fj::Circle {
    fn bounding_volume(&self) -> Aabb {
        Aabb {
            min: point![-self.radius, -self.radius, 0.0],
            max: point![self.radius, self.radius, 0.0],
        }
    }

    fn faces(&self, tolerance: f64) -> Faces {
        let vertices: Vec<_> = self
            .edges()
            .0
            .into_iter()
            .map(|edge| edge.vertices(tolerance))
            .flatten()
            .collect();
        triangulate(&vertices)
    }

    fn edges(&self) -> Edges {
        let mut edges = Edges::new();
        edges.0.push(Edge::arc(self.radius));
        edges
    }

    fn vertices(&self) -> Vec<Point> {
        // Circles have just a single round edge with no vertices.
        Vec::new()
    }
}
