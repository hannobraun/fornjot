use fj_math::Transform;

use crate::{
    operations::insert::Insert,
    storage::Handle,
    topology::{HalfEdge, Surface},
    Core,
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
        let (half_edge, _) = self;

        let curve = half_edge
            .curve()
            .clone()
            .transform_with_cache(transform, core, cache);
        let start_vertex = half_edge
            .start_vertex()
            .clone()
            .transform_with_cache(transform, core, cache);

        let transformed_half_edge =
            HalfEdge::new(curve, start_vertex).insert(core);

        core.layers.geometry.define_half_edge(
            transformed_half_edge.clone(),
            *core.layers.geometry.of_half_edge(half_edge),
        );

        transformed_half_edge
    }
}
