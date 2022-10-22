use fj_math::Transform;

use crate::{
    objects::{Objects, Solid},
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for Handle<Solid> {
    fn transform(self, transform: &Transform, objects: &Objects) -> Self {
        let faces = self
            .shells()
            .cloned()
            .map(|shell| shell.transform(transform, objects));
        Solid::builder(objects).with_shells(faces).build()
    }
}
