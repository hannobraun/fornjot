use fj_math::Transform;

use crate::objects::{Objects, Solid};

use super::TransformObject;

impl TransformObject for Solid {
    fn transform(self, transform: &Transform, objects: &Objects) -> Self {
        let faces = self
            .into_shells()
            .map(|shell| shell.transform(transform, objects));
        Self::builder(objects).with_shells(faces).build()
    }
}
