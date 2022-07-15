use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    objects::Solid,
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::Aabb;

use super::Shape;

impl Shape for fj::Group {
    type Brep = Solid;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let mut faces = Vec::new();

        let a = self.a.compute_brep(config, tolerance, debug_info)?;
        let b = self.b.compute_brep(config, tolerance, debug_info)?;

        faces.extend(a.into_inner().into_faces());
        faces.extend(b.into_inner().into_faces());

        let group = Solid::from_faces(faces);
        validate(group, config)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}
