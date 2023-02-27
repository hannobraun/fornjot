use crate::{
    insert::Insert,
    objects::{Cycle, HalfEdge, Objects},
    services::Service,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<Cycle> {
    fn reverse(self, objects: &mut Service<Objects>) -> Self {
        let mut edges = self
            .half_edges()
            .cloned()
            .map(|edge| {
                let boundary = {
                    let [a, b] = edge.boundary();
                    [b, a]
                };
                let surface_vertices = {
                    let [a, b] = edge.surface_vertices().map(Clone::clone);
                    [b, a]
                };

                HalfEdge::new(
                    edge.curve(),
                    boundary,
                    surface_vertices,
                    edge.global_form().clone(),
                )
                .insert(objects)
            })
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges).insert(objects)
    }
}
