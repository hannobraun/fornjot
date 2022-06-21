use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    shape::Shape,
    validation::{self, validate, Validated, ValidationError},
};
use fj_math::Aabb;

use super::ToShape;

impl ToShape for fj::Group {
    fn to_shape(
        &self,
        config: &validation::ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Shape>, ValidationError> {
        let mut shape = Shape::new();

        let a = self.a.to_shape(config, tolerance, debug_info)?;
        let b = self.b.to_shape(config, tolerance, debug_info)?;

        shape.merge_shape(&a)?;
        shape.merge_shape(&b)?;

        let shape = validate(shape, config)?;

        Ok(shape)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}
