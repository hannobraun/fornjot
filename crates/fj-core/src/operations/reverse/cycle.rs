use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert},
    topology::{Cycle, HalfEdge},
};

use super::Reverse;

impl Reverse for Cycle {
    fn reverse(&self, core: &mut Core) -> Self {
        let mut edges = self
            .half_edges()
            .pairs()
            .map(|(current, next)| {
                HalfEdge::new(
                    current.curve().clone(),
                    next.start_vertex().clone(),
                )
                .insert(core)
                .derive_from(current, core)
            })
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges)
    }
}
