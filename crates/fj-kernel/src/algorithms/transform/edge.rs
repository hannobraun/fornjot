use fj_math::Transform;

use crate::{
    objects::{GlobalEdge, HalfEdge},
    partial::{HasPartial, PartialGlobalEdge},
    stores::Stores,
};

use super::TransformObject;

impl TransformObject for HalfEdge {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let curve = self.curve().clone().transform(transform, stores);
        let vertices = self
            .vertices()
            // The `clone` can be replaced with `each_ref`, once that is stable:
            // https://doc.rust-lang.org/std/primitive.array.html#method.each_ref
            .clone()
            .map(|vertex| {
                vertex
                    .to_partial()
                    .transform(transform, stores)
                    .with_curve(curve.clone())
                    .build(stores)
            });
        let global_form = self
            .global_form()
            .to_partial()
            .transform(transform, stores)
            .with_curve(curve.global_form().clone())
            .build(stores);

        Self::new(curve, vertices, global_form)
    }
}

impl TransformObject for GlobalEdge {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let curve = self.curve().clone().transform(transform, stores);
        let vertices = self
            .vertices()
            .map(|vertex| vertex.transform(transform, stores));

        Self::new(curve, vertices)
    }
}

impl TransformObject for PartialGlobalEdge {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let curve =
            self.curve.map(|curve| curve.0.transform(transform, stores));
        let vertices = self.vertices.map(|vertices| {
            vertices.map(|vertex| vertex.transform(transform, stores))
        });

        Self {
            curve: curve.map(Into::into),
            vertices,
        }
    }
}
