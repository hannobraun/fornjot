use crate::{
    operations::{
        derive::DeriveFrom, geometry::UpdateHalfEdgeGeometry, insert::Insert,
    },
    topology::{Cycle, HalfEdge},
    Core,
};

use super::Reverse;

impl Reverse for Cycle {
    fn reverse(&self, core: &mut Core) -> Self {
        let mut edges = self
            .half_edges()
            .pairs()
            .map(|(current, next)| {
                let mut half_edge_geom =
                    *core.layers.geometry.of_half_edge(current);
                half_edge_geom.boundary = half_edge_geom.boundary.reverse();

                HalfEdge::new(
                    current.curve().clone(),
                    next.start_vertex().clone(),
                )
                .insert(core)
                .derive_from(current, core)
                .set_geometry(half_edge_geom, &mut core.layers.geometry)
            })
            .collect::<Vec<_>>();

        edges.reverse();

        Cycle::new(edges)
    }
}
