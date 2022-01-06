use parry3d_f64::bounding_volume::{BoundingVolume as _, AABB};

use crate::{
    debug::DebugInfo,
    kernel::{
        topology::{edges::Edges, faces::Faces},
        Shape,
    },
    math::Point,
};

impl Shape for fj::Union {
    fn bounding_volume(&self) -> AABB {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }

    fn faces(&self, tolerance: f64, debug_info: &mut DebugInfo) -> Faces {
        let a = self.a.faces(tolerance, debug_info);
        let b = self.b.faces(tolerance, debug_info);

        // TASK: This doesn't create a true union, as it doesn't eliminate,
        //       merge faces, or split faces.
        let mut faces = Vec::new();
        faces.extend(a.0);
        faces.extend(b.0);

        Faces(faces)
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
