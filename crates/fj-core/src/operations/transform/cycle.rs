use fj_math::Transform;

use crate::{objects::Cycle, Instance};

use super::{TransformCache, TransformObject};

impl TransformObject for Cycle {
    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Instance,
        cache: &mut TransformCache,
    ) -> Self {
        let edges = self.half_edges().iter().map(|edge| {
            edge.clone().transform_with_cache(transform, core, cache)
        });

        Self::new(edges)
    }
}
