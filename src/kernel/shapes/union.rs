use crate::{
    debug::DebugInfo,
    kernel::topology::{edges::Edges, faces::Faces, vertices::Vertices, Shape},
    math::{Aabb, Scalar},
};

use super::ToShape;

impl ToShape for fj::Union {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
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

        let faces = Faces(faces);

        Shape {
            edges: Edges { cycles: Vec::new() },
            faces,
        }
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }

    fn vertices(&self) -> Vertices {
        todo!()
    }
}
