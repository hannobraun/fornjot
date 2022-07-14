use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{transform_faces, Tolerance},
    objects::Face,
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Transform, Vector};

use super::Shape;

impl Shape for fj::Transform {
    type Brep = Vec<Face>;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let mut shape = self
            .shape
            .compute_brep(config, tolerance, debug_info)?
            .into_inner();

        transform_faces(&mut shape, &make_transform(self));

        validate(shape, config)
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
