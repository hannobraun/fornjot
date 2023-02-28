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
        // Don't need to transform curve, as that's defined in surface
        // coordinates.
        let curve = self.curve();
        let boundary = self.boundary();
        let start_vertex = self
            .start_vertex()
            .clone()
            .transform_with_cache(transform, objects, cache);
        let end_vertex = self
            .end_vertex()
            .clone()
            .transform_with_cache(transform, objects, cache);
        let global_form = self
            .global_form()
            .clone()
            .transform_with_cache(transform, objects, cache);

        Self::new(curve, boundary, start_vertex, end_vertex, global_form)
    }
}

impl TransformObject for GlobalEdge {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let vertices =
            self.vertices().access_in_normalized_order().map(|vertex| {
                vertex.transform_with_cache(transform, objects, cache)
            });

        Self::new(vertices)
    }
}
