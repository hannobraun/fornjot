use parry3d_f64::bounding_volume::{BoundingVolume as _, AABB};

use crate::{
    kernel::{edges::Edges, faces::Faces, Shape},
    math::Point,
};

impl Shape for fj::Union {
    fn bounding_volume(&self) -> AABB {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }

    fn faces(&self, tolerance: f64) -> Faces {
        let a = self.a.faces(tolerance);
        let b = self.b.faces(tolerance);

        let mut triangles = Vec::new();
        a.triangles(&mut triangles);
        b.triangles(&mut triangles);

        Faces::Triangles(triangles)
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
