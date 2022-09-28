use fj_math::Transform;

use crate::{objects::Surface, stores::Stores};

use super::TransformObject;

impl TransformObject for Surface {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        Self::new(
            self.u().transform(transform, stores),
            transform.transform_vector(&self.v()),
        )
    }
}
