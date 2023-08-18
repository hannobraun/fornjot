use crate::{objects::Edge, services::Services};

use super::ReverseCurveCoordinateSystems;

impl ReverseCurveCoordinateSystems for Edge {
    fn reverse_curve_coordinate_systems(&self, _: &mut Services) -> Self {
        let path = self.path().reverse();
        let boundary = self.boundary().reverse();

        Edge::new(
            path,
            boundary,
            self.curve().clone(),
            self.start_vertex().clone(),
        )
    }
}
