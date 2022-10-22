use fj_math::Transform;

use crate::{
    objects::{Objects, Shell},
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for Handle<Shell> {
    fn transform(self, transform: &Transform, objects: &Objects) -> Self {
        let faces = self
            .faces()
            .clone()
            .into_iter()
            .map(|face| face.transform(transform, objects));
        Shell::builder(objects).with_faces(faces).build()
    }
}
