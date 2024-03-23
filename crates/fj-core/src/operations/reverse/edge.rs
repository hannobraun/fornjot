use crate::{
    objects::HalfEdge,
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    Core,
};

use super::ReverseCurveCoordinateSystems;

impl ReverseCurveCoordinateSystems for Handle<HalfEdge> {
    fn reverse_curve_coordinate_systems(&self, core: &mut Core) -> Self {
        let mut geometry = core.layers.geometry.of_half_edge(self);
        geometry.path = geometry.path.reverse();
        geometry.boundary = geometry.boundary.reverse();

        let half_edge = HalfEdge::new(
            geometry.boundary,
            self.curve().clone(),
            self.start_vertex().clone(),
        )
        .insert(core)
        .derive_from(self, core);

        core.layers
            .geometry
            .define_half_edge(half_edge.clone(), geometry);

        half_edge
    }
}
