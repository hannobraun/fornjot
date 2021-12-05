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
            mins: point![-self.radius, -self.radius, 0.0],
            maxs: point![self.radius, self.radius, 0.0],
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
        Faces(triangulate(&vertices))
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
