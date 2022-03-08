use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::Surface,
        shape::{faces::Faces, Shape},
        topology::{
            edges::{Cycle, Edge},
            faces::Face,
        },
    },
    math::{Aabb, Point, Scalar},
};

use super::ToShape;

impl ToShape for fj::Circle {
    fn to_shape(&self, _: Scalar, _: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();

        // Circles have just a single round edge with no vertices. So none need
        // to be added here.

        let edge = shape
            .edges()
            .add(Edge::circle(Scalar::from_f64(self.radius)));
        shape.cycles().add(Cycle { edges: vec![edge] });

        let cycles = shape
            .cycles()
            .all()
            .map(|handle| (*handle).clone())
            .collect();
        *shape.faces() = Faces(vec![Face::Face {
            cycles,
            surface: Surface::x_y_plane(),
        }]);

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb {
            min: Point::from([-self.radius, -self.radius, 0.0]),
            max: Point::from([self.radius, self.radius, 0.0]),
        }
    }
}
