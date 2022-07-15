use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{Tolerance, TransformObject},
    objects::Solid,
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Transform, Vector};

use super::Shape;

impl Shape for fj::Transform {
    type Brep = Solid;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let original = Solid::from_faces(
            self.shape
                .compute_brep(config, tolerance, debug_info)?
                .into_inner(),
        );

        let transformed = original.transform(&make_transform(self));
        validate(transformed, config)
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
