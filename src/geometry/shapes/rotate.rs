use parry3d_f64::{bounding_volume::AABB, math::Isometry};

use crate::{
    geometry::{edges::Edges, faces::Faces, Shape},
    math::{Point, Vector},
};

impl Shape for fj::Rotate {
    fn bounding_volume(&self) -> AABB {
        let axis = Vector::from(self.axis).normalize();

        self.shape
            .bounding_volume()
            .transform_by(&Isometry::rotation(axis * self.angle))
    }

    fn faces(&self, _tolerance: f64) -> Faces {
        // TASK: Implement.
        todo!()
    }

    fn edges(&self) -> Edges {
        // TASK: Implement.
        todo!()
    }

    fn vertices(&self) -> Vec<Point> {
        // TASK: Implement.
        todo!()
    }
}
