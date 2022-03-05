use crate::{
    debug::DebugInfo,
    kernel::{algorithms::sweep::sweep_shape, shape::Shape},
    math::{Aabb, Scalar, Vector},
};

use super::ToShape;

impl ToShape for fj::Sweep {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        sweep_shape(
            self.shape.to_shape(tolerance, debug_info),
            Vector::from([0., 0., self.length]),
            tolerance,
        )
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let mut aabb = self.shape.bounding_volume();
        aabb.max.z = self.length.into();
        aabb
    }
}
