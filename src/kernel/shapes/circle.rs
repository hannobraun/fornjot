use nalgebra::point;
use parry3d_f64::bounding_volume::AABB;

use crate::{
    kernel::{
        edges::{Edge, Edges},
        faces::Faces,
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

    fn faces(&self, _: f64) -> Faces {
        // TASK: Implement.
        todo!()
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
