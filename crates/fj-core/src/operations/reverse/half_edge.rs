use crate::{
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    topology::HalfEdge,
    Core,
};

use super::ReverseCurveCoordinateSystems;

impl ReverseCurveCoordinateSystems for Handle<HalfEdge> {
    fn reverse_curve_coordinate_systems(&self, core: &mut Core) -> Self {
        let mut half_edge_geom = *core.layers.geometry.of_half_edge(self);
        half_edge_geom.path = half_edge_geom.path.reverse();
        half_edge_geom.boundary = half_edge_geom.boundary.reverse();

        let half_edge =
            HalfEdge::new(self.curve().clone(), self.start_vertex().clone())
                .insert(core)
                .derive_from(self, core);

        core.layers
            .geometry
            .define_half_edge(half_edge.clone(), half_edge_geom);

        half_edge
    }
}
