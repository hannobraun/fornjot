use fj_math::Transform;

use crate::{objects::Shell, stores::Stores};

use super::TransformObject;

impl TransformObject for Shell {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let faces = self
            .into_faces()
            .into_iter()
            .map(|face| face.transform(transform, stores));
        Self::new().with_faces(faces)
    }
}
