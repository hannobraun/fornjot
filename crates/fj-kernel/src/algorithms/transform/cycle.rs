use fj_math::Transform;

use crate::{
    objects::{Cycle, Objects},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Cycle {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let surface = self
            .surface()
            .clone()
            .transform_with_cache(transform, objects, cache);
        let half_edges = self.half_edges().map(|half_edge| {
            half_edge
                .clone()
                .transform_with_cache(transform, objects, cache)
        });

        Self::new(surface, half_edges)
    }
}
