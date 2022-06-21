use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{sweep_shape, Tolerance},
    shape::Shape,
    validation::{self, validate, Validated, ValidationError},
};
use fj_math::{Aabb, Vector};

use super::ToShape;

impl ToShape for fj::Sweep {
    fn to_shape(
        &self,
        config: &validation::ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Shape>, ValidationError> {
        let shape = self.shape().to_shape(config, tolerance, debug_info)?;
        let path = Vector::from(self.path());
        let color = self.shape().color();

        let swept = sweep_shape(shape.into_inner(), path, tolerance, color)?;
        let swept = validate(swept, config)?;

        Ok(swept)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        self.shape()
            .bounding_volume()
            .merged(&Aabb::<3>::from_points(
                self.shape()
                    .bounding_volume()
                    .vertices()
                    .map(|v| v + self.path()),
            ))
    }
}
