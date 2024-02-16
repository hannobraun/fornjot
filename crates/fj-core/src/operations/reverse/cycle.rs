use crate::{
    objects::{Cycle, HalfEdge},
    operations::insert::Insert,
    Core,
};

use super::{Reverse, ReverseCurveCoordinateSystems};

impl Reverse for Cycle {
    fn reverse(&self, core: &mut Core) -> Self {
        let mut edges = self
            .half_edges()
            .pairs()
            .map(|(current, next)| {
                HalfEdge::new(
                    current.path(),
                    current.boundary().reverse(),
                    current.curve().clone(),
                    next.start_vertex().clone(),
                )
                .insert(core)
            })
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges)
    }
}

impl ReverseCurveCoordinateSystems for Cycle {
    fn reverse_curve_coordinate_systems(&self, core: &mut Core) -> Self {
        let edges = self.half_edges().iter().map(|edge| {
            edge.reverse_curve_coordinate_systems(core).insert(core)
        });

        Cycle::new(edges)
    }
}
