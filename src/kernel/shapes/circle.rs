use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::Surface,
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
            vertices::Vertices,
            Shape,
        },
    },
    math::{Aabb, Point, Scalar},
};

use super::ToShape;

impl ToShape for fj::Circle {
    fn to_shape(&self, _: Scalar, _: &mut DebugInfo) -> Shape {
        // Circles have just a single round edge with no vertices.
        let vertices = Vertices(Vec::new());

        let edges = Edges::single_cycle([Edge::circle(self.radius)]);

        let faces = Faces(vec![Face::Face {
            edges: edges.clone(),
            surface: Surface::x_y_plane(),
        }]);

        Shape {
            vertices,
            edges,
            faces,
        }
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb {
            min: Point::from([-self.radius, -self.radius, 0.0]),
            max: Point::from([self.radius, self.radius, 0.0]),
        }
    }
}
