use fj_math::Transform;

use crate::{objects::Objects, partial::PartialCycle, services::Service};

use super::{TransformCache, TransformObject};

impl TransformObject for PartialCycle {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let half_edges = self.half_edges().map(|edge| {
            edge.into_partial()
                .transform_with_cache(transform, objects, cache)
        });

        Self::default().with_half_edges(half_edges)
    }
}
