use fj_math::Transform;

use crate::{
    objects::{Objects, Vertex},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Vertex {
    fn transform_with_cache(
        self,
        transform: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        let position = transform.transform_point(&self.position());
        Self::new(position)
    }
}
