use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    algorithms::{
        approx::Tolerance,
        sweep::Sweep,
        validate::{Validate, Validated, ValidationConfig, ValidationError},
    },
    objects::Solid,
};
use fj_math::{Aabb, Vector};

use super::Shape;

impl Shape for fj::Sweep {
    type Brep = Solid;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let sketch =
            self.shape().compute_brep(config, tolerance, debug_info)?;
        let path = Vector::from(self.path());
        let color = self.shape().color();

        let solid = sketch.into_inner().sweep(path, tolerance, Color(color));
        solid.validate_with_config(config)
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
