use fj_math::Transform;

use crate::{
    operations::insert::Insert,
    storage::Handle,
    topology::{Cycle, Surface},
    Core,
};

use super::{TransformCache, TransformObject};

impl TransformObject for (&Handle<Cycle>, &Handle<Surface>) {
    type Transformed = Handle<Cycle>;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let (cycle, _) = self;

        let half_edges = cycle.half_edges().iter().map(|half_edge| {
            half_edge.transform_with_cache(transform, core, cache)
        });

        Cycle::new(half_edges).insert(core)
    }
}
