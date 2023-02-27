use itertools::Itertools;

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
            .circular_tuple_windows()
            .map(|(current, next)| {
                let boundary = {
                    let [a, b] = current.boundary();
                    [b, a]
                };
                let surface_vertices =
                    [next.start_vertex(), current.start_vertex()]
                        .map(Clone::clone);

                HalfEdge::new(
                    current.curve(),
                    boundary,
                    surface_vertices,
                    current.global_form().clone(),
                )
                .insert(objects)
            })
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges).insert(objects)
    }
}
