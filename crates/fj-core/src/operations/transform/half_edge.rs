use fj_math::Transform;

use crate::{
    Core,
    operations::insert::Insert,
    storage::Handle,
    topology::{HalfEdge, Surface},
};

use super::{TransformCache, TransformObject};

impl TransformObject for (&Handle<HalfEdge>, &Handle<Surface>) {
    type Transformed = Handle<HalfEdge>;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let (half_edge, surface) = self;

        let curve = (half_edge.curve(), surface)
            .transform_with_cache(transform, core, cache);
        let start_vertex = half_edge
            .start_vertex()
            .clone()
            .transform_with_cache(transform, core, cache);

        HalfEdge::new(curve, start_vertex).insert(core)
    }
}
