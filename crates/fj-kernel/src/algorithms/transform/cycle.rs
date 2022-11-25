use fj_math::Transform;

use crate::{objects::Objects, partial::PartialCycle, services::Service};

use super::TransformObject;

impl TransformObject for PartialCycle {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let half_edges = self
            .half_edges()
            .map(|edge| edge.into_partial().transform(transform, objects));

        Self::default().with_half_edges(half_edges)
    }
}
