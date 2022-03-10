use crate::{
    debug::DebugInfo,
    kernel::shape::Shape,
    math::{Aabb, Scalar},
};

use super::ToShape;

impl ToShape for fj::Union {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();

        let mut a = self.a.to_shape(tolerance, debug_info);
        let mut b = self.b.to_shape(tolerance, debug_info);

        // This doesn't create a true union, as it doesn't eliminate, merge, or
        // split faces.
        //
        // See issue:
        // https://github.com/hannobraun/Fornjot/issues/42
        for face in a.faces().faces() {
            shape.faces().add_face(face.get().clone()).unwrap();
        }
        for face in b.faces().faces() {
            shape.faces().add_face(face.get().clone()).unwrap();
        }

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}
