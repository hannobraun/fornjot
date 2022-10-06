use fj_math::Transform;

use crate::{
    objects::Curve,
    partial::{MaybePartial, PartialGlobalEdge, PartialHalfEdge},
    stores::{Handle, Stores},
};

use super::TransformObject;

impl TransformObject for PartialHalfEdge {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let surface = self
            .surface
            .map(|surface| surface.transform(transform, stores));
        let curve = self.curve.clone().map(|curve| {
            curve
                .into_partial()
                .transform(transform, stores)
                .with_surface(surface.clone())
                .into()
        });
        let vertices = self.vertices.clone().map(|vertices| {
            vertices.map(|vertex| {
                vertex
                    .into_partial()
                    .transform(transform, stores)
                    .with_curve(curve.clone())
                    .into()
            })
        });
        let global_form = self.global_form.map(|global_form| {
            global_form
                .into_partial()
                .transform(transform, stores)
                .with_curve(curve.as_ref().and_then(
                    |curve: &MaybePartial<Handle<Curve>>| curve.global_form(),
                ))
                .into()
        });

        Self {
            surface,
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
