use fj_math::Transform;

use crate::{
    objects::{GlobalVertex, SurfaceVertex, Vertex},
    stores::Stores,
};

use super::TransformObject;

impl TransformObject for Vertex {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let curve = self.curve().clone().transform(transform, stores);
        let surface_form = self.surface_form().transform(transform, stores);
        let global_form = self.global_form().transform(transform, stores);

        // Don't need to transform `self.position`, as that is in curve
        // coordinates and thus transforming the curve takes care of it.
        Self::new(self.position(), curve, surface_form, global_form)
    }
}

impl TransformObject for SurfaceVertex {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        Self::new(
            self.position(),
            self.surface().transform(transform, stores),
            self.global_form().transform(transform, stores),
        )
    }
}

impl TransformObject for GlobalVertex {
    fn transform(self, transform: &Transform, _: &Stores) -> Self {
        let position = transform.transform_point(&self.position());
        Self::from_position(position)
    }
}
