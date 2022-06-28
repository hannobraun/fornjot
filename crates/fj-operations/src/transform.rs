use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{transform, Tolerance},
    objects::Face,
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Transform, Vector};

use super::ToShape;

impl ToShape for fj::Transform {
    fn to_shape(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Vec<Face>>, ValidationError> {
        let shape = self.shape.to_shape(config, tolerance, debug_info)?;
        let shape = shape.into_inner();

        let faces = transform(&shape, &make_transform(self));

        let shape = validate(faces, config)?;

        Ok(shape)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        make_transform(self).transform_aabb(&self.shape.bounding_volume())
    }
}

fn make_transform(transform: &fj::Transform) -> Transform {
    let axis = Vector::from(transform.axis).normalize();
    Transform::translation(transform.offset)
        * Transform::rotation(axis * transform.angle.rad())
}
