use parry3d_f64::{bounding_volume::AABB, math::Isometry};

use crate::{
    debug::DebugInfo,
    kernel::{
        math::{Transform, Vector},
        topology::{edges::Edges, faces::Faces, vertices::Vertices},
        Shape,
    },
};

impl Shape for fj::Transform {
    fn bounding_volume(&self) -> AABB {
        self.shape.bounding_volume().transform_by(&isometry(self))
    }

    fn faces(&self, tolerance: f64, debug_info: &mut DebugInfo) -> Faces {
        self.shape
            .faces(tolerance, debug_info)
            .transform(&isometry(self))
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vertices {
        todo!()
    }
}

fn isometry(transform: &fj::Transform) -> Transform {
    let axis = Vector::from(transform.axis).normalize();
    Isometry::new(Vector::from(transform.offset), axis * transform.angle)
}
