use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::Surface,
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
            vertices::Vertices,
        },
    },
    math::{Aabb, Point, Scalar},
};

use super::ToShape;

impl ToShape for fj::Circle {
    fn bounding_volume(&self) -> Aabb<3> {
        Aabb {
            min: Point::from([-self.radius, -self.radius, 0.0]),
            max: Point::from([self.radius, self.radius, 0.0]),
        }
    }

    fn faces(&self, _: Scalar, _: &mut DebugInfo) -> Faces {
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
