use fj_interop::debug::DebugInfo;
use fj_kernel::{algorithms::Tolerance, shape::Shape};
use fj_math::Aabb;

use super::ToShape;

impl ToShape for fj::Group {
    fn to_shape(
        &self,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Shape {
        let mut shape = Shape::new();

        let a = self.a.to_shape(tolerance, debug_info);
        let b = self.b.to_shape(tolerance, debug_info);

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

fn copy_shape(orig: Shape, target: &mut Shape) {
    for face_orig in orig.faces() {
        target.merge(face_orig.get()).unwrap();
    }
}
