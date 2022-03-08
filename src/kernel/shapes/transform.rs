use parry3d_f64::math::Isometry;

use crate::{
    debug::DebugInfo,
    kernel::{algorithms::transform::transform_shape, shape::Shape},
    math::{Aabb, Scalar, Transform},
};

use super::ToShape;

impl ToShape for fj::Transform {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        let shape = self.shape.to_shape(tolerance, debug_info);
        let transform = transform(self);
        transform_shape(shape, &transform)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        transform(self).transform_aabb(&self.shape.bounding_volume())
    }
}

fn transform(transform: &fj::Transform) -> Transform {
    let axis = nalgebra::Vector::from(transform.axis).normalize();
    Isometry::new(
        nalgebra::Vector::from(transform.offset),
        axis * transform.angle,
    )
    .into()
}
