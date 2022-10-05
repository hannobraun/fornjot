use fj_math::Transform;

use crate::{objects::Cycle, partial::PartialCycle, stores::Stores};

use super::TransformObject;

impl TransformObject for Cycle {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let surface = self.surface().clone().transform(transform, stores);
        let half_edges = self
            .into_half_edges()
            .map(|edge| edge.transform(transform, stores));

        Self::new(surface, half_edges)
    }
}

impl TransformObject for PartialCycle {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let surface = self
            .surface
            .clone()
            .map(|surface| surface.transform(transform, stores));
        let half_edges = self
            .half_edges
            .into_iter()
            .map(|edge| edge.transform(transform, stores))
            .collect();

        Self {
            surface,
            half_edges,
        }
    }
}
