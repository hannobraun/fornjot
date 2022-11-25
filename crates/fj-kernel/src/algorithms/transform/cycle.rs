use fj_math::Transform;

use crate::{
    objects::Objects, partial::PartialCycle, services::Service,
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for PartialCycle {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Result<Self, ValidationError> {
        let half_edges = self
            .half_edges()
            .map(|edge| edge.into_partial().transform(transform, objects))
            .collect::<Result<Vec<_>, ValidationError>>()?;

        Ok(Self::default().with_half_edges(half_edges))
    }
}
