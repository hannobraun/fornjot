use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::transform::TransformObject,
    objects::{FaceSet, Objects},
    validate::ValidationError,
};
use fj_math::{Aabb, Transform, Vector};

use super::Shape;

impl Shape for fj::Transform {
    type Brep = FaceSet;

    fn compute_brep(
        &self,
        objects: &Objects,
        debug_info: &mut DebugInfo,
    ) -> Result<Self::Brep, ValidationError> {
        let faces = self
            .shape
            .compute_brep(objects, debug_info)?
            .transform(&make_transform(self), objects)?;

        Ok(faces)
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
