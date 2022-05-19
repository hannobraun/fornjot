use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{sweep_shape, Tolerance},
    shape::Shape,
};
use fj_math::{Aabb, Point, Vector};

use super::ToShape;

impl ToShape for fj::Sweep {
    fn to_shape(
        &self,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Shape {
        sweep_shape(
            self.shape().to_shape(tolerance, debug_info),
            Vector::from(self.path()),
            tolerance,
            self.shape().color(),
        )
        .unwrap()
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let target = Point::origin() + self.path();
        self.shape().bounding_volume().include_point(&target)
    }
}
