use fj_math::Transform;

use crate::{objects::Solid, stores::Stores};

use super::TransformObject;

impl TransformObject for Solid {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let faces = self
            .into_shells()
            .map(|shell| shell.transform(transform, stores));
        Self::new().with_shells(faces)
    }
}
