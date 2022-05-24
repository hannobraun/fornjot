use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{sweep_shape, Tolerance},
    shape::{Shape, ValidationError},
};
use fj_math::{Aabb, Vector};

use super::ToShape;

impl ToShape for fj::Sweep {
    fn to_shape(
        &self,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Shape, ValidationError> {
        sweep_shape(
            self.shape().to_shape(tolerance, debug_info)?,
            Vector::from(self.path()),
            tolerance,
            self.shape().color(),
        )
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
