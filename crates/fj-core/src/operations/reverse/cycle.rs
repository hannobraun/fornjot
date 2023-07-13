use crate::{
    objects::{Cycle, HalfEdge},
    operations::Insert,
    services::Services,
};

use super::Reverse;

impl Reverse for Cycle {
    fn reverse(&self, services: &mut Services) -> Self {
        let mut edges = self
            .half_edge_pairs()
            .map(|(current, next)| {
                let boundary = {
                    let [a, b] = current.boundary();
                    [b, a]
                };

                HalfEdge::new(
                    current.path(),
                    boundary,
                    next.start_vertex().clone(),
                    current.global_form().clone(),
                )
                .insert(services)
            })
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges)
    }
}
