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
        let vertices = self
            .vertices
            .try_map_ext(|vertex| vertex.transform(transform, objects))?;
        let global_form = self.global_form.transform(transform, objects)?;

        Ok(Self {
            curve,
            vertices,
            global_form,
        })
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
