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
        let (half_edge, surface) = self;

        let curve = (half_edge.curve(), surface)
            .transform_with_cache(transform, core, cache);
        let start_vertex = half_edge
            .start_vertex()
            .clone()
            .transform_with_cache(transform, core, cache);

        let transformed_half_edge =
            HalfEdge::new(curve, start_vertex).insert(core);

        if let Some(half_edge_geom) =
            core.layers.geometry.of_half_edge(half_edge)
        {
            core.layers.geometry.define_half_edge(
                transformed_half_edge.clone(),
                *half_edge_geom,
            );
        }

        transformed_half_edge
    }
}
