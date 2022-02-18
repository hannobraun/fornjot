use crate::{
    debug::DebugInfo,
    kernel::{
        topology::{edges::Edges, faces::Faces, vertices::Vertices},
        Shape,
    },
    math::{Aabb, Scalar},
};

impl Shape for fj::Difference {
    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }

    fn faces(&self, _tolerance: Scalar, _: &mut DebugInfo) -> Faces {
        todo!()
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vertices {
        todo!()
    }
}
