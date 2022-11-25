use fj_math::Transform;

use crate::{
    objects::Objects,
    partial::{PartialGlobalVertex, PartialSurfaceVertex, PartialVertex},
    services::Service,
};

use super::TransformObject;

impl TransformObject for PartialVertex {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let curve = self.curve.transform(transform, objects);
        let surface_form = self
            .surface_form
            .into_partial()
            .transform(transform, objects);

        // Don't need to transform `self.position`, as that is in curve
        // coordinates and thus transforming the curve takes care of it.
        Self {
            position: self.position,
            curve,
            surface_form: surface_form.into(),
        }
    }
}

impl TransformObject for PartialSurfaceVertex {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let surface = self
            .surface
            .clone()
            .map(|surface| surface.transform(transform, objects));
        let global_form = self.global_form.transform(transform, objects);

        // Don't need to transform `self.position`, as that is in surface
        // coordinates and thus transforming the surface takes care of it.
        Self {
            position: self.position,
            surface,
            global_form,
        }
    }
}

impl TransformObject for PartialGlobalVertex {
    fn transform(
        self,
        transform: &Transform,
        _: &mut Service<Objects>,
    ) -> Self {
        let position = self
            .position
            .map(|position| transform.transform_point(&position));

        Self { position }
    }
}
