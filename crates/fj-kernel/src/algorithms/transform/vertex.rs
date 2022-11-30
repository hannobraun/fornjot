use fj_math::Transform;

use crate::{
    objects::{GlobalVertex, Objects, SurfaceVertex, Vertex},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Vertex {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        // Don't need to transform position, as that is defined in curve
        // coordinates and thus transforming the curve takes care of it.
        let position = self.position();

        let curve = self
            .curve()
            .clone()
            .transform_with_cache(transform, objects, cache);
        let surface_form = self
            .surface_form()
            .clone()
            .transform_with_cache(transform, objects, cache);

        Self::new(position, curve, surface_form)
    }
}

impl TransformObject for SurfaceVertex {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        // Don't need to transform position, as that is defined in surface
        // coordinates and thus transforming the surface takes care of it.
        let position = self.position();

        let surface = self
            .surface()
            .clone()
            .transform_with_cache(transform, objects, cache);
        let global_form = self
            .global_form()
            .clone()
            .transform_with_cache(transform, objects, cache);

        Self::new(position, surface, global_form)
    }
}

impl TransformObject for GlobalVertex {
    fn transform_with_cache(
        self,
        transform: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        let position = transform.transform_point(&self.position());
        Self::from_position(position)
    }
}
