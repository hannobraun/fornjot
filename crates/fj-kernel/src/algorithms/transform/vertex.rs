use fj_math::Transform;

use crate::{
    objects::Objects,
    partial::{PartialGlobalVertex, PartialSurfaceVertex, PartialVertex},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for PartialVertex {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let curve = self.curve.transform_with_cache(transform, objects, cache);
        let surface_form = self
            .surface_form
            .into_partial()
            .transform_with_cache(transform, objects, cache);

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
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let surface = self.surface.clone().map(|surface| {
            surface.transform_with_cache(transform, objects, cache)
        });
        let global_form = self
            .global_form
            .transform_with_cache(transform, objects, cache);

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
    fn transform_with_cache(
        self,
        transform: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        let position = self
            .position
            .map(|position| transform.transform_point(&position));

        Self { position }
    }
}
