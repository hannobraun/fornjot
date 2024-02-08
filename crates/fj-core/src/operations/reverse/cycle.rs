use crate::{
    objects::{Cycle, HalfEdge},
    operations::insert::Insert,
    Instance,
};

use super::{Reverse, ReverseCurveCoordinateSystems};

impl Reverse for Cycle {
    fn reverse(&self, core: &mut Instance) -> Self {
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
                .insert(&mut core.services)
            })
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges)
    }
}

impl ReverseCurveCoordinateSystems for Cycle {
    fn reverse_curve_coordinate_systems(&self, core: &mut Instance) -> Self {
        let edges = self.half_edges().iter().map(|edge| {
            edge.reverse_curve_coordinate_systems(core)
                .insert(&mut core.services)
        });

        Cycle::new(edges)
    }
}
