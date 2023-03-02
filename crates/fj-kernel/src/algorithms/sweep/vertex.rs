use fj_math::Vector;

use crate::{
    insert::Insert,
    objects::{GlobalEdge, Objects, Vertex},
    services::Service,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Vertex> {
    type Swept = (Handle<GlobalEdge>, [Self; 2]);

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &mut Service<Objects>,
    ) -> Self::Swept {
        let a = self.clone();
        let b = cache
            .global_vertex
            .entry(self.id())
            .or_insert_with(|| {
                Vertex::new(self.position() + path.into()).insert(objects)
            })
            .clone();

        let vertices = [a, b];
        let global_edge = GlobalEdge::new(vertices.clone()).insert(objects);

        // The vertices of the returned `GlobalEdge` are in normalized order,
        // which means the order can't be relied upon by the caller. Return the
        // ordered vertices in addition.
        (global_edge, vertices)
    }
}
