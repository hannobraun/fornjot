use fj_math::Transform;

use crate::{topology::Cycle, Core};

use super::{TransformCache, TransformObject};

impl TransformObject for Cycle {
    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self {
        let half_edges = self.half_edges().iter().map(|edge| {
            edge.clone().transform_with_cache(transform, core, cache)
        });

        Self::new(half_edges)
    }
}
