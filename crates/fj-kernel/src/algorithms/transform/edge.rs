use fj_math::Transform;

use crate::{
    partial::{PartialGlobalEdge, PartialHalfEdge},
    stores::Stores,
};

use super::TransformObject;

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
