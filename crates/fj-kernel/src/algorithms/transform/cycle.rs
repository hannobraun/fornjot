use fj_math::Transform;

use crate::{objects::Objects, partial::PartialCycle};

use super::TransformObject;

impl TransformObject for PartialCycle {
    fn transform(self, transform: &Transform, stores: &Objects) -> Self {
        let surface = self
            .surface
            .clone()
            .map(|surface| surface.transform(transform, stores));
        let half_edges = self
            .half_edges
            .into_iter()
            .map(|edge| {
                edge.into_partial()
                    .transform(transform, stores)
                    .with_surface(surface.clone())
                    .into()
            })
            .collect();

        Self {
            surface,
            half_edges,
        }
    }
}
