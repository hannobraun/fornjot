use fj_math::Vector;

use crate::{
    objects::{GlobalEdge, Vertex},
    operations::Insert,
    services::Services,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Vertex> {
    type Swept = (Handle<GlobalEdge>, [Self; 2]);

    fn sweep_with_cache(
        self,
        _: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> Self::Swept {
        let a = self.clone();
        let b = cache
            .vertices
            .entry(self.id())
            .or_insert_with(|| Vertex::new().insert(services))
            .clone();

        let vertices = [a, b];
        let global_edge = cache
            .global_edge
            .entry(self.id())
            .or_insert_with(|| GlobalEdge::new().insert(services))
            .clone();

        // The vertices of the returned `GlobalEdge` are in normalized order,
        // which means the order can't be relied upon by the caller. Return the
        // ordered vertices in addition.
        (global_edge, vertices)
    }
}
