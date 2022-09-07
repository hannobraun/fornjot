use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{
        approx::Tolerance,
        validate::{Validate, Validated, ValidationConfig, ValidationError},
    },
    objects::Faces,
};
use fj_math::Aabb;

use super::Shape;

impl Shape for fj::Group {
    type Brep = Faces;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let mut faces = Faces::new();

        let a = self.a.compute_brep(config, tolerance, debug_info)?;
        let b = self.b.compute_brep(config, tolerance, debug_info)?;

        faces.extend(a.into_inner());
        faces.extend(b.into_inner());

        faces.validate_with_config(config)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}
