use fj_math::Transform;

use crate::{
    objects::{GlobalEdge, HalfEdge},
    partial::{HasPartial, PartialGlobalEdge, PartialHalfEdge},
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
            .vertices_in_normalized_order()
            .map(|vertex| vertex.transform(transform, stores));

        Self::new(curve, vertices)
    }
}

impl TransformObject for PartialHalfEdge {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let curve = self
            .curve
            .clone()
            .map(|curve| curve.transform(transform, stores));
        let vertices = self.vertices.clone().map(|vertices| {
            vertices.map(|vertex| {
                let vertex = vertex.into_partial().transform(transform, stores);
                let vertex = match &curve {
                    Some(curve) => vertex.with_curve(curve.clone()),
                    None => vertex,
                };
                vertex.into()
            })
        });
        let global_form = self.global_form.map(|global_form| {
            let global_form =
                global_form.into_partial().transform(transform, stores);

            let curve = curve.as_ref().and_then(|curve| curve.global_form());
            let global_form = match curve {
                Some(curve) => global_form.with_curve(curve),
                None => global_form,
            };

            global_form.into()
        });

        Self {
            curve,
            vertices,
            global_form,
        }
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
