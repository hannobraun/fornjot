use fj_math::Transform;

use crate::{
    objects::{GlobalVertex, SurfaceVertex, Vertex},
    stores::Stores,
};

use super::TransformObject;

impl TransformObject for Vertex {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        Self::new(
            self.position(),
            self.curve().clone().transform(transform, stores),
            self.surface_form().transform(transform, stores),
            self.global_form().transform(transform, stores),
        )
    }
}

impl TransformObject for SurfaceVertex {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        Self::new(
            self.position(),
            self.surface().transform(transform, stores),
            self.global_form().transform(transform, stores),
        )
    }
}

impl TransformObject for GlobalVertex {
    type Transformed = Self;

    fn transform(self, transform: &Transform, _: &Stores) -> Self {
        let position = transform.transform_point(&self.position());
        Self::from_position(position)
    }
}
