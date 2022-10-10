use fj_math::Transform;

use crate::{
    objects::{Curve, Objects},
    partial::{MaybePartial, PartialGlobalEdge, PartialHalfEdge},
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for PartialHalfEdge {
    fn transform(self, transform: &Transform, objects: &Objects) -> Self {
        let surface = self
            .surface
            .map(|surface| surface.transform(transform, objects));
        let curve = self.curve.clone().map(|curve| {
            curve
                .into_partial()
                .transform(transform, objects)
                .with_surface(surface.clone())
                .into()
        });
        let vertices = self.vertices.clone().map(|vertices| {
            vertices.map(|vertex| {
                vertex
                    .into_partial()
                    .transform(transform, objects)
                    .with_curve(curve.clone())
                    .into()
            })
        });
        let global_form = self.global_form.map(|global_form| {
            global_form
                .into_partial()
                .transform(transform, objects)
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
    fn transform(self, transform: &Transform, objects: &Objects) -> Self {
        let curve = self
            .curve
            .map(|curve| curve.0.transform(transform, objects));
        let vertices = self.vertices.map(|vertices| {
            vertices.map(|vertex| vertex.transform(transform, objects))
        });

        Self {
            curve: curve.map(Into::into),
            vertices,
        }
    }
}
