use fj_math::Transform;

use crate::objects::{Objects, Solid};

use super::TransformObject;

impl TransformObject for Solid {
    fn transform(self, transform: &Transform, stores: &Objects) -> Self {
        let faces = self
            .into_shells()
            .map(|shell| shell.transform(transform, stores));
        Self::new().with_shells(faces)
    }
}
