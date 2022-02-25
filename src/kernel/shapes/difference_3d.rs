use crate::{
    debug::DebugInfo,
    kernel::topology::{edges::Edges, faces::Faces, vertices::Vertices, Shape},
    math::{Aabb, Scalar},
};

use super::ToShape;

impl ToShape for fj::Difference {
    fn to_shape(&self, _: Scalar, _: &mut DebugInfo) -> Shape {
        Shape {
            vertices: Vertices(Vec::new()),
            edges: Edges { cycles: Vec::new() },
            faces: Faces(Vec::new()),
        }
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }
}
