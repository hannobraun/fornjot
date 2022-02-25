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
        let edges = self.edges();
        let faces = Faces(vec![Face::Face {
            edges,
            surface: Surface::x_y_plane(),
        }]);

        Shape { faces }
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb {
            min: Point::from([-self.radius, -self.radius, 0.0]),
            max: Point::from([self.radius, self.radius, 0.0]),
        }
    }

    fn edges(&self) -> Edges {
        Edges::single_cycle([Edge::circle(self.radius)])
    }

    fn vertices(&self) -> Vertices {
        // Circles have just a single round edge with no vertices.
        Vertices(Vec::new())
    }
}
