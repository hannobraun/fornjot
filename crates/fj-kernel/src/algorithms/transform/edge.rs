use fj_interop::ext::ArrayExt;
use fj_math::Transform;

use crate::{
    objects::Objects,
    partial::{MaybePartial, PartialGlobalEdge, PartialHalfEdge},
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for PartialHalfEdge {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let curve: MaybePartial<_> = self
            .curve()
            .into_partial()
            .transform(transform, objects)?
            .into();
        let vertices = self.vertices().try_map_ext(
            |vertex| -> Result<_, ValidationError> {
                Ok(vertex
                    .into_partial()
                    .transform(transform, objects)?
                    .with_curve(Some(curve.clone())))
            },
        )?;
        let global_form = self
            .global_form()
            .into_partial()
            .transform(transform, objects)?
            .with_curve(curve.global_form())
            .into();

        Ok(Self::default()
            .with_curve(curve)
            .with_vertices(Some(vertices))
            .with_global_form(global_form))
    }
}

impl TransformObject for PartialGlobalEdge {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let curve = self.curve().transform(transform, objects)?;
        let vertices = self
            .vertices()
            .map(|vertices| {
                vertices.try_map_ext(|vertex| -> Result<_, ValidationError> {
                    vertex.transform(transform, objects)
                })
            })
            .transpose()?;

        Ok(Self::default()
            .with_curve(Some(curve))
            .with_vertices(vertices))
    }
}
