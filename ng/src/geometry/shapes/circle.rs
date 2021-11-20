use nalgebra::point;

use crate::{
    geometry::{
        bounding_volume::Aabb,
        edges::Edge,
        faces::{triangulate, Triangle},
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

    fn edges(&self, _: f64) -> Vec<Edge> {
        vec![Edge::arc(self.radius)]
    }

    fn triangles(&self, tolerance: f64) -> Vec<Triangle> {
        let vertices: Vec<_> = self
            .edges(tolerance)
            .into_iter()
            .map(|edge| edge.vertices(tolerance))
            .flatten()
            .collect();
        triangulate(&vertices)
    }

    fn vertices(&self) -> Vec<Point> {
        // Circles have just a single round edge with no vertices.
        Vec::new()
    }
}
