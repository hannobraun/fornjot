use nalgebra::point;

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::Surface,
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
            vertices::Vertices,
        },
        Shape,
    },
    math::Aabb,
};

impl Shape for fj::Circle {
    fn bounding_volume(&self) -> Aabb<3> {
        Aabb {
            min: point![-self.radius, -self.radius, 0.0],
            max: point![self.radius, self.radius, 0.0],
        }
    }

    fn faces(&self, _: f64, _: &mut DebugInfo) -> Faces {
        let edges = Edges::single_cycle([Edge::circle(self.radius)]);
        Faces(vec![Face::Face {
            edges,
            surface: Surface::x_y_plane(),
        }])
    }

    fn edges(&self) -> Edges {
        Edges::single_cycle([Edge::circle(self.radius)])
    }

    fn vertices(&self) -> Vertices {
        // Circles have just a single round edge with no vertices.
        Vertices(Vec::new())
    }
}
