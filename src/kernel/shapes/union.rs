use parry3d_f64::bounding_volume::{BoundingVolume as _, AABB};

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry,
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

    fn faces(
        &self,
        tolerance: f64,
        cache: &mut geometry::Cache,
        debug_info: &mut DebugInfo,
    ) -> Faces {
        let a = self.a.faces(tolerance, cache, debug_info);
        let b = self.b.faces(tolerance, cache, debug_info);

        // This doesn't create a true union, as it doesn't eliminate, merge, or
        // split faces.
        //
        // See issue:
        // https://github.com/hannobraun/Fornjot/issues/42
        let mut faces = Vec::new();
        faces.extend(a.0);
        faces.extend(b.0);

        Faces(faces)
    }

    fn edges(&self, _: &mut geometry::Cache) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vec<Point<3>> {
        todo!()
    }
}
