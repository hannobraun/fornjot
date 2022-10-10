use fj_math::Transform;

use crate::objects::{Shell, Stores};

use super::TransformObject;

impl TransformObject for Shell {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let faces = self
            .into_faces()
            .into_iter()
            .map(|face| face.transform(transform, stores));
        Self::new().with_faces(faces)
    }
}
