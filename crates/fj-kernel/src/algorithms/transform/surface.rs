use fj_math::Transform;

use crate::{
    objects::{Objects, Surface},
    storage::Handle,
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for Handle<Surface> {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        Ok(objects.surfaces.insert(Surface::new(
            self.geometry().u.transform(transform),
            transform.transform_vector(&self.geometry().v),
        ))?)
    }
}
