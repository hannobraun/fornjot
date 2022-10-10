use fj_math::Transform;

use crate::{
    objects::{Objects, Surface},
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for Handle<Surface> {
    fn transform(self, transform: &Transform, stores: &Objects) -> Self {
        stores.surfaces.insert(Surface::new(
            self.u().transform(transform, stores),
            transform.transform_vector(&self.v()),
        ))
    }
}
