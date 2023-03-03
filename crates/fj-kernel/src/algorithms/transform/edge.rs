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
        let global_form = self
            .global_form()
            .clone()
            .transform_with_cache(transform, objects, cache);

        Self::new(curve, boundary, start_vertex, global_form)
    }
}

impl TransformObject for GlobalEdge {
    fn transform_with_cache(
        self,
        _: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        // There's nothing to actually transform here, as `GlobalEdge` holds no
        // data. We still need this implementation though, as a new `GlobalEdge`
        // must be created to represent the new and transformed edge.
        Self::new()
    }
}
