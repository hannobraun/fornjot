use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{sweep_shape, Tolerance},
    shape::Shape,
};
use fj_math::{Aabb, Vector};

use super::ToShape;

impl ToShape for fj::Sweep {
    fn to_shape(
        &self,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Shape {
        sweep_shape(
            self.shape().to_shape(tolerance, debug_info),
            Vector::from([0., 0., self.length()]),
            tolerance,
            self.shape().color(),
        )
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let mut aabb = self.shape().bounding_volume();
        aabb.max.z = self.length().into();
        aabb
    }
}
