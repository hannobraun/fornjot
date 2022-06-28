use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    objects::Face,
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::Aabb;

use super::ToShape;

impl ToShape for fj::Group {
    fn to_shape(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Vec<Face>>, ValidationError> {
        let mut shape = Vec::new();

        let a = self.a.to_shape(config, tolerance, debug_info)?;
        let b = self.b.to_shape(config, tolerance, debug_info)?;

        shape.extend(a.into_inner());
        shape.extend(b.into_inner());

        let shape = validate(shape, config)?;

        Ok(shape)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}
