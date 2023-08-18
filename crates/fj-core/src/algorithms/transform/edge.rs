use fj_math::Transform;

use crate::{objects::HalfEdge, services::Services};

use super::{TransformCache, TransformObject};

impl TransformObject for HalfEdge {
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        // Don't need to transform the path, as that's defined in surface
        // coordinates.
        let path = self.path();
        let boundary = self.boundary();
        let curve = self
            .curve()
            .clone()
            .transform_with_cache(transform, services, cache);
        let start_vertex = self
            .start_vertex()
            .clone()
            .transform_with_cache(transform, services, cache);

        Self::new(path, boundary, curve, start_vertex)
    }
}
