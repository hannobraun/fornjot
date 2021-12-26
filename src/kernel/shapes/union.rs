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
        let Faces::Faces(a) = self.a.faces(tolerance);
        let Faces::Faces(b) = self.b.faces(tolerance);

        // TASK: This doesn't create a true union, as it doesn't eliminate,
        //       merge faces, or split faces.
        let mut faces = Vec::new();
        faces.extend(a);
        faces.extend(b);

        Faces::Faces(faces)
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
