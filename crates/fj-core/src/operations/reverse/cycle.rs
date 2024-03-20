use crate::{
    geometry::HalfEdgeGeometry,
    objects::{Cycle, HalfEdge},
    operations::{derive::DeriveFrom, insert::Insert},
    Core,
};

use super::{Reverse, ReverseCurveCoordinateSystems};

impl Reverse for Cycle {
    fn reverse(&self, core: &mut Core) -> Self {
        let mut edges = self
            .half_edges()
            .pairs()
            .map(|(current, next)| {
                let half_edge = HalfEdge::new(
                    core.layers.geometry.of_half_edge(current).path,
                    current.boundary().reverse(),
                    current.curve().clone(),
                    next.start_vertex().clone(),
                )
                .insert(core)
                .derive_from(current, core);

                core.layers.geometry.define_half_edge(
                    half_edge.clone(),
                    HalfEdgeGeometry {
                        path: core.layers.geometry.of_half_edge(current).path,
                    },
                );

                half_edge
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
