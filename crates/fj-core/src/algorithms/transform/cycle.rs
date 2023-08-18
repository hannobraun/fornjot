use fj_math::Transform;

use crate::{objects::Cycle, services::Services};

use super::{TransformCache, TransformObject};

impl TransformObject for Cycle {
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        let half_edges = self.edges().map(|half_edge| {
            half_edge
                .clone()
                .transform_with_cache(transform, services, cache)
        });

        Self::new(half_edges)
    }
}
