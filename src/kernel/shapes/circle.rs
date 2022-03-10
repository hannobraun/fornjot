use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::Surface,
        shape::Shape,
        topology::{edges::Cycle, faces::Face},
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
            .add_circle(Scalar::from_f64(self.radius))
            .unwrap();
        shape.cycles().add(Cycle { edges: vec![edge] }).unwrap();

        let cycles = shape.cycles().all().collect();
        let surface =
            shape.geometry().add_surface(Surface::x_y_plane()).unwrap();
        shape.faces().add(Face::Face { cycles, surface }).unwrap();

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb {
            min: Point::from([-self.radius, -self.radius, 0.0]),
            max: Point::from([self.radius, self.radius, 0.0]),
        }
    }
}
