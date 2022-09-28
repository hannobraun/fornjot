use fj_math::Transform;

use crate::{objects::Cycle, stores::Stores};

use super::TransformObject;

impl TransformObject for Cycle {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        Self::new(
            self.surface().transform(transform, stores),
            self.into_half_edges()
                .map(|edge| edge.transform(transform, stores)),
        )
    }
}
