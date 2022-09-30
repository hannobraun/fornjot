use fj_math::Transform;

use crate::{
    objects::Surface,
    stores::{Handle, Stores},
};

use super::TransformObject;

impl TransformObject for Handle<Surface> {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        stores.surfaces.insert(Surface::new(
            self.u().transform(transform, stores),
            transform.transform_vector(&self.v()),
        ))
    }
}
