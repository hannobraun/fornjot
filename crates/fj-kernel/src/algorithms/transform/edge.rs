use fj_interop::ext::ArrayExt;
use fj_math::Transform;

use crate::{
    objects::Objects,
    partial::{PartialGlobalEdge, PartialHalfEdge},
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for PartialHalfEdge {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let curve = self.curve.transform(transform, objects)?;
        let vertices = self.vertices.try_map_ext(
            |vertex| -> Result<_, ValidationError> {
                let mut vertex =
                    vertex.into_partial().transform(transform, objects)?;
                vertex.curve = curve.clone();
                Ok(vertex)
            },
        )?;
        let mut global_form = self
            .global_form
            .into_partial()
            .transform(transform, objects)?;
        global_form.curve = curve.global_form();

        Ok(Self::default()
            .with_curve(curve)
            .with_vertices(vertices)
            .with_global_form(global_form))
    }
}

impl TransformObject for PartialGlobalEdge {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let curve = self.curve.transform(transform, objects)?;
        let vertices = self.vertices.try_map_ext(
            |vertex| -> Result<_, ValidationError> {
                vertex.transform(transform, objects)
            },
        )?;

        Ok(Self { curve, vertices })
    }
}
