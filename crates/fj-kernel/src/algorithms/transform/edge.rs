use fj_math::Transform;

use crate::{
    objects::{GlobalEdge, HalfEdge, Objects, Vertex},
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
        let curve = self
            .curve()
            .clone()
            .transform_with_cache(transform, objects, cache);
        let vertices = self.vertices().map(|vertex| {
            let point = vertex.position();
            let surface_form = vertex
                .surface_form()
                .clone()
                .transform_with_cache(transform, objects, cache);

            Vertex::new(point, surface_form)
        });
        let global_form = self
            .global_form()
            .clone()
            .transform_with_cache(transform, objects, cache);

        Self::new(curve, vertices, global_form)
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
