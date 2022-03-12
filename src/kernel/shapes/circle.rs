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
            .topology()
            .add_circle(Scalar::from_f64(self.radius()))
            .unwrap();
        shape
            .topology()
            .add_cycle(Cycle { edges: vec![edge] })
            .unwrap();

        let cycles = shape.topology().cycles().collect();
        let surface = shape.geometry().add_surface(Surface::x_y_plane());
        shape
            .topology()
            .add_face(Face::Face { cycles, surface })
            .unwrap();

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb {
            min: Point::from([-self.radius(), -self.radius(), 0.0]),
            max: Point::from([self.radius(), self.radius(), 0.0]),
        }
    }
}
