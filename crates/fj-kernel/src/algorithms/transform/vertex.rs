use fj_math::Transform;

use crate::{
    objects::Objects,
    partial::{PartialGlobalVertex, PartialSurfaceVertex, PartialVertex},
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for PartialVertex {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let curve = self
            .curve
            .map(|curve| curve.transform(transform, objects))
            .transpose()?;
        let surface_form = self
            .surface_form
            .map(|surface_form| -> Result<_, ValidationError> {
                Ok(surface_form
                    .into_partial()
                    .transform(transform, objects)?
                    .into())
            })
            .transpose()?;
        let global_form = self
            .global_form
            .map(|global_form| global_form.transform(transform, objects))
            .transpose()?;

        // Don't need to transform `self.position`, as that is in curve
        // coordinates and thus transforming the curve takes care of it.
        Ok(Self {
            position: self.position,
            curve,
            surface_form,
            global_form,
        })
    }
}

impl TransformObject for PartialSurfaceVertex {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let surface = self
            .surface
            .map(|surface| surface.transform(transform, objects))
            .transpose()?;
        let global_form = self
            .global_form
            .map(|global_form| global_form.transform(transform, objects))
            .transpose()?;

        // Don't need to transform `self.position`, as that is in surface
        // coordinates and thus transforming the surface takes care of it.
        Ok(Self {
            position: self.position,
            surface,
            global_form,
        })
    }
}

impl TransformObject for PartialGlobalVertex {
    fn transform(
        self,
        transform: &Transform,
        _: &Objects,
    ) -> Result<Self, ValidationError> {
        let position = self
            .position
            .map(|position| transform.transform_point(&position));

        Ok(Self { position })
    }
}
