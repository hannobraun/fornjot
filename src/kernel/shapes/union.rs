use crate::{
    debug::DebugInfo,
    kernel::topology::{faces::Faces, Shape},
    math::{Aabb, Scalar},
};

use super::ToShape;

impl ToShape for fj::Union {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();

        let a = self.a.to_shape(tolerance, debug_info).faces;
        let b = self.b.to_shape(tolerance, debug_info).faces;

        // This doesn't create a true union, as it doesn't eliminate, merge, or
        // split faces.
        //
        // See issue:
        // https://github.com/hannobraun/Fornjot/issues/42
        let mut faces = Vec::new();
        faces.extend(a.0);
        faces.extend(b.0);

        shape.faces = Faces(faces);

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}
