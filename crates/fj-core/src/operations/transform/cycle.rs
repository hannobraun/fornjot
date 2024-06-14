use fj_math::Transform;

use crate::{topology::Cycle, Core};

use super::{TransformCache, TransformObject};

impl TransformObject for Cycle {
    type Transformed = Self;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let cycle = self;

        let half_edges = cycle.half_edges().iter().map(|half_edge| {
            half_edge
                .clone()
                .transform_with_cache(transform, core, cache)
        });

        Self::new(half_edges)
    }
}
