use crate::{
    objects::{Cycle, Edge},
    operations::Insert,
    services::Services,
};

use super::{Reverse, ReverseCurveCoordinateSystems};

impl Reverse for Cycle {
    fn reverse(&self, services: &mut Services) -> Self {
        let mut edges = self
            .half_edge_pairs()
            .map(|(current, next)| {
                Edge::new(
                    current.path(),
                    current.boundary().reverse(),
                    current.curve().clone(),
                    next.start_vertex().clone(),
                )
                .insert(services)
            })
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges)
    }
}

impl ReverseCurveCoordinateSystems for Cycle {
    fn reverse_curve_coordinate_systems(
        &self,
        services: &mut Services,
    ) -> Self {
        let edges = self.edges().map(|edge| {
            edge.reverse_curve_coordinate_systems(services)
                .insert(services)
        });

        Cycle::new(edges)
    }
}
