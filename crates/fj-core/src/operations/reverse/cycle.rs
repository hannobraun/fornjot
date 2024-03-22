use crate::{
    objects::{Cycle, HalfEdge},
    operations::{
        derive::DeriveFrom, geometry::UpdateHalfEdgeGeometry, insert::Insert,
    },
    Core,
};

use super::{Reverse, ReverseCurveCoordinateSystems};

impl Reverse for Cycle {
    fn reverse(&self, core: &mut Core) -> Self {
        let mut edges = self
            .half_edges()
            .pairs()
            .map(|(current, next)| {
                let path = core.layers.geometry.of_half_edge(current).path;

                HalfEdge::new(
                    current.boundary().reverse(),
                    current.curve().clone(),
                    next.start_vertex().clone(),
                )
                .insert(core)
                .derive_from(current, core)
                .set_path(path, &mut core.layers.geometry)
            })
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges)
    }
}

impl ReverseCurveCoordinateSystems for Cycle {
    fn reverse_curve_coordinate_systems(&self, core: &mut Core) -> Self {
        let edges = self
            .half_edges()
            .iter()
            .map(|edge| edge.reverse_curve_coordinate_systems(core));

        Cycle::new(edges)
    }
}
