use crate::{
    debug::DebugInfo,
    kernel::shape::Shape,
    math::{Aabb, Scalar},
};

use super::ToShape;

impl ToShape for fj::Union {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();

        let a = self.a.to_shape(tolerance, debug_info);
        let b = self.b.to_shape(tolerance, debug_info);

        // This doesn't create a true union, as it doesn't eliminate, merge, or
        // split faces.
        //
        // See issue:
        // https://github.com/hannobraun/Fornjot/issues/42
        copy_shape(a, &mut shape);
        copy_shape(b, &mut shape);

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}

fn copy_shape(mut orig: Shape, target: &mut Shape) {
    for face in orig.topology().faces() {
        target.topology().add_face(face.get().clone()).unwrap();
    }
}
