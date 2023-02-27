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
        // Don't need to transform curve, as that's defined in surface
        // coordinates.
        let curve = self.curve();
        let boundary = self.boundary();
        let surface_vertices = self.surface_vertices().map(|surface_vertex| {
            surface_vertex
                .clone()
                .transform_with_cache(transform, objects, cache)
        });
        let boundary = boundary.zip_ext(surface_vertices);
        let global_form = self
            .global_form()
            .clone()
            .transform_with_cache(transform, objects, cache);

        Self::new(curve, boundary, global_form)
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
