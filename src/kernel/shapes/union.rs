use crate::{
    debug::DebugInfo,
    kernel::{
        topology::{edges::Edges, faces::Faces, vertices::Vertices},
        Shape,
    },
    math::Aabb,
};

impl Shape for fj::Union {
    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }

    fn faces(&self, tolerance: f64, debug_info: &mut DebugInfo) -> Faces {
        let a = self.a.faces(tolerance, debug_info);
        let b = self.b.faces(tolerance, debug_info);

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

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vertices {
        todo!()
    }
}
