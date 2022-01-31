use parry3d_f64::{bounding_volume::AABB, math::Isometry};

use crate::{
    debug::DebugInfo,
    kernel::{
        topology::{edges::Edges, faces::Faces},
        Shape,
    },
    math::{Point, Vector},
};

impl Shape for fj::Transform {
    fn bounding_volume(&self) -> AABB {
        self.shape.bounding_volume().transform_by(&isometry(self))
    }

    fn faces(&self, tolerance: f64, debug_info: &mut DebugInfo) -> Faces {
        let isometry = isometry(self);
        self.shape.faces(tolerance, debug_info).transform(&isometry)
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vec<Point<3>> {
        todo!()
    }
}

fn isometry(transform: &fj::Transform) -> Isometry<f64> {
    let axis = Vector::from(transform.axis).normalize();
    Isometry::new(Vector::from(transform.offset), axis * transform.angle)
}
