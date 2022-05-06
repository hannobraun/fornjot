use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    geometry::Surface,
    shape::Shape,
    topology::{Cycle, Edge, Face},
};
use fj_math::{Aabb, Point, Scalar};

use super::ToShape;

impl ToShape for fj::Circle {
    fn to_shape(&self, _: Tolerance, _: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();

        // Circles have just a single round edge with no vertices. So none need
        // to be added here.

        let edge = Edge::builder(&mut shape)
            .build_circle(Scalar::from_f64(self.radius()))
            .unwrap();
        shape.insert(Cycle { edges: vec![edge] }).unwrap();

        let cycles = shape.cycles().collect();
        let surface = shape.insert(Surface::x_y_plane()).unwrap();
        shape
            .insert(Face::Face {
                exteriors: cycles,
                interiors: Vec::new(),
                surface,
                color: self.color(),
            })
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
