use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::transform::TransformObject,
    objects::{FaceSet, Objects},
    validate::{Validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Transform, Vector};

use super::Shape;

impl Shape for fj::Transform {
    type Brep = FaceSet;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        objects: &Objects,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let faces = self
            .shape
            .compute_brep(config, objects, debug_info)?
            .into_inner()
            .transform(&make_transform(self), objects)?;

        faces.validate_with_config(config)
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
