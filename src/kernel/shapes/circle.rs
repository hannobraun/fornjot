use nalgebra::point;
use parry3d_f64::bounding_volume::AABB;

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::{self, Surface},
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
        },
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

    fn faces(
        &self,
        _: f64,
        _: &mut geometry::Cache,
        _: &mut DebugInfo,
    ) -> Faces {
        let edges = Edges::single_cycle([Edge::arc(self.radius)]);
        Faces(vec![Face::Face {
            edges,
            surface: Surface::x_y_plane(),
        }])
    }

    fn edges(&self, _: &mut geometry::Cache) -> Edges {
        Edges::single_cycle([Edge::arc(self.radius)])
    }

    fn vertices(&self) -> Vec<Point<3>> {
        // Circles have just a single round edge with no vertices.
        Vec::new()
    }
}
