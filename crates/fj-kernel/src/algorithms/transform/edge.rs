use fj_math::Transform;

use crate::{
    objects::{GlobalEdge, HalfEdge},
    stores::Stores,
};

use super::TransformObject;

impl TransformObject for HalfEdge {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let curve = self.curve().clone().transform(transform, stores);
        let vertices = self
            .vertices()
            .clone()
            .map(|vertex| vertex.transform(transform, stores));
        let global_form =
            self.global_form().clone().transform(transform, stores);

        Self::new(curve, vertices, global_form)
    }
}

impl TransformObject for GlobalEdge {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let curve = self.curve().clone().transform(transform, stores);
        let vertices = self
            .vertices()
            .map(|vertex| vertex.transform(transform, stores));

        Self::new(curve, vertices)
    }
}
