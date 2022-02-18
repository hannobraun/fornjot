use parry3d_f64::math::Isometry;

use crate::{
    debug::DebugInfo,
    kernel::{
        topology::{edges::Edges, faces::Faces, vertices::Vertices},
        Shape,
    },
    math::{Aabb, Transform},
};

impl Shape for fj::Transform {
    fn bounding_volume(&self) -> Aabb<3> {
        transform(self).transform_aabb(&self.shape.bounding_volume())
    }

    fn faces(&self, tolerance: f64, debug_info: &mut DebugInfo) -> Faces {
        self.shape
            .faces(tolerance, debug_info)
            .transform(&transform(self))
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vertices {
        todo!()
    }
}

fn transform(transform: &fj::Transform) -> Transform {
    let axis = nalgebra::Vector::from(transform.axis).normalize();
    Isometry::new(
        nalgebra::Vector::from(transform.offset),
        axis * transform.angle,
    )
    .into()
}
