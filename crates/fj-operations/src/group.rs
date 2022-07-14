use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    objects::Face,
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::Aabb;

use super::Shape;

impl Shape for fj::Group {
    type Brep = Vec<Face>;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let mut shape = Vec::new();

        let a = self.a.compute_brep(config, tolerance, debug_info)?;
        let b = self.b.compute_brep(config, tolerance, debug_info)?;

        shape.extend(a.into_inner());
        shape.extend(b.into_inner());

        validate(shape, config)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}
