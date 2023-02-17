use fj_interop::ext::ArrayExt;
use fj_math::Transform;

use crate::{
    objects::{GlobalEdge, HalfEdge, Objects},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for HalfEdge {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let surface = self
            .surface()
            .clone()
            .transform_with_cache(transform, objects, cache);
        let curve = self
            .curve()
            .clone()
            .transform_with_cache(transform, objects, cache);
        let boundary = self.boundary().zip_ext(self.surface_vertices()).map(
            |(point, surface_vertex)| {
                let surface_vertex = surface_vertex
                    .clone()
                    .transform_with_cache(transform, objects, cache);
                (point, surface_vertex)
            },
        );
        let global_form = self
            .global_form()
            .clone()
            .transform_with_cache(transform, objects, cache);

        Self::new(surface, curve, boundary, global_form)
    }
}

impl TransformObject for GlobalEdge {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let curve = self
            .curve()
            .clone()
            .transform_with_cache(transform, objects, cache);
        let vertices =
            self.vertices().access_in_normalized_order().map(|vertex| {
                vertex.transform_with_cache(transform, objects, cache)
            });

        Self::new(curve, vertices)
    }
}
