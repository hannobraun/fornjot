use fj_math::Transform;

use crate::{
    objects::{Objects, Surface},
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for Handle<Surface> {
    fn transform(self, transform: &Transform, objects: &Objects) -> Self {
        objects.surfaces.insert(Surface::new(
            self.u().transform(transform, objects),
            transform.transform_vector(&self.v()),
        ))
    }
}
