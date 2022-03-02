use crate::{
    debug::DebugInfo,
    kernel::{algorithms::sweep::sweep_shape, topology::Shape},
    math::{Aabb, Scalar},
};

use super::ToShape;

impl ToShape for fj::Sweep {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        sweep_shape(
            &self.shape.to_shape(tolerance, debug_info),
            self.length,
            tolerance,
        )
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let mut aabb = self.shape.bounding_volume();
        aabb.max.z = self.length.into();
        aabb
    }
}
