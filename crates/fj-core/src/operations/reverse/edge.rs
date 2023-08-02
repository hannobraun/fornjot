use crate::{objects::HalfEdge, services::Services};

use super::ReverseCurveCoordinateSystems;

impl ReverseCurveCoordinateSystems for HalfEdge {
    fn reverse_curve_coordinate_systems(&self, _: &mut Services) -> Self {
        let path = self.path().reverse();
        let boundary = self.boundary().reverse();

        HalfEdge::new(
            path,
            boundary,
            self.curve().clone(),
            self.start_vertex().clone(),
            self.global_form().clone(),
        )
    }
}
