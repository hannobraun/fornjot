use crate::{
    objects::HalfEdge,
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    Core,
};

use super::ReverseCurveCoordinateSystems;

impl ReverseCurveCoordinateSystems for Handle<HalfEdge> {
    fn reverse_curve_coordinate_systems(&self, core: &mut Core) -> Self {
        let path = self.path().reverse();
        let boundary = self.boundary().reverse();

        HalfEdge::new(
            path,
            boundary,
            self.curve().clone(),
            self.start_vertex().clone(),
        )
        .insert(core)
        .derive_from(self, core)
    }
}
