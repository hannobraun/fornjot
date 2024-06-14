use fj_math::Transform;

use crate::{
    operations::insert::Insert, storage::Handle, topology::HalfEdge, Core,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Handle<HalfEdge> {
    type Transformed = Self;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let curve = self
            .curve()
            .clone()
            .transform_with_cache(transform, core, cache);
        let start_vertex = self
            .start_vertex()
            .clone()
            .transform_with_cache(transform, core, cache);

        let half_edge = HalfEdge::new(curve, start_vertex).insert(core);

        core.layers.geometry.define_half_edge(
            half_edge.clone(),
            *core.layers.geometry.of_half_edge(&self),
        );

        half_edge
    }
}
