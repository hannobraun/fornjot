use crate::{
    operations::{
        derive::DeriveFrom, geometry::UpdateHalfEdgeGeometry, insert::Insert,
    },
    topology::{Cycle, HalfEdge},
    Core,
};

use super::{Reverse, ReverseCurveCoordinateSystems};

impl Reverse for Cycle {
    fn reverse(&self, core: &mut Core) -> Self {
        let mut edges = self
            .half_edges()
            .pairs()
            .map(|(current, next)| {
                let mut geometry = *core.layers.geometry.of_half_edge(current);
                geometry.boundary = geometry.boundary.reverse();

                HalfEdge::new(
                    current.curve().clone(),
                    next.start_vertex().clone(),
                )
                .insert(core)
                .derive_from(current, core)
                .set_geometry(geometry, &mut core.layers.geometry)
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
