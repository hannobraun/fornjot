use nalgebra::point;
use parry3d_f64::bounding_volume::AABB;

use crate::{
    kernel::{
        edges::{Edge, Edges},
        faces::{triangulate, Face, Faces},
        Shape,
    },
    math::Point,
};

impl Shape for fj::Circle {
    fn bounding_volume(&self) -> AABB {
        AABB {
            mins: point![-self.radius, -self.radius, 0.0],
            maxs: point![self.radius, self.radius, 0.0],
        }
    }

    fn faces(&self, tolerance: f64) -> Faces {
        let vertices: Vec<_> = self
            .edges()
            .0
            .into_iter()
            .map(|edge| edge.approx_vertices(tolerance))
            .flatten()
            .collect();
        let triangles = triangulate(&vertices);
        let faces = vec![Face(triangles)];

        Faces(faces)
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
