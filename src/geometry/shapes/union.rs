use parry3d_f64::bounding_volume::{BoundingVolume as _, AABB};

use crate::{
    geometry::{edges::Edges, faces::Faces, Shape},
    math::Point,
};

impl Shape for fj::Union {
    fn bounding_volume(&self) -> AABB {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
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
