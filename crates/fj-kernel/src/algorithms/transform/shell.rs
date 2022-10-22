use fj_math::Transform;

use crate::objects::{Objects, Shell};

use super::TransformObject;

impl TransformObject for Shell {
    fn transform(self, transform: &Transform, objects: &Objects) -> Self {
        let faces = self
            .faces()
            .clone()
            .into_iter()
            .map(|face| face.transform(transform, objects));
        Self::builder(objects).with_faces(faces).build()
    }
}
