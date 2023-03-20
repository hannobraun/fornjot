use fj_interop::debug::DebugInfo;
use fj_kernel::{
    objects::{Face, Set},
    services::Services,
};
use fj_math::Aabb;

use super::Shape;

impl Shape for fj::Group {
    type Brep = Set<Face>;

    fn compute_brep(
        &self,
        services: &mut Services,
        debug_info: &mut DebugInfo,
    ) -> Self::Brep {
        let mut faces = Set::new();

        let a = self.a.compute_brep(services, debug_info);
        let b = self.b.compute_brep(services, debug_info);

        faces.extend(a);
        faces.extend(b);

        faces
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}
