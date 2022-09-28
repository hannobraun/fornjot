use fj_math::Transform;

use crate::{objects::Sketch, stores::Stores};

use super::TransformObject;

impl TransformObject for Sketch {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let faces = self
            .into_faces()
            .into_iter()
            .map(|face| face.transform(transform, stores));
        Self::new().with_faces(faces)
    }
}
