use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::Surface,
        shape::{edges::Edges, Shape},
        topology::faces::{Face, Faces},
    },
    math::{Aabb, Point, Scalar},
};

use super::ToShape;

impl ToShape for fj::Circle {
    fn to_shape(&self, _: Scalar, _: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();

        // Circles have just a single round edge with no vertices. So none need
        // to be added here.

        *shape.edges() = Edges::single_cycle([shape
            .edges()
            .create_circle(Scalar::from_f64(self.radius))]);

        shape.faces = Faces(vec![Face::Face {
            edges: shape.edges().clone(),
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
