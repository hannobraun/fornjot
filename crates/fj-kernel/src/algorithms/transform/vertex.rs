use fj_math::Transform;

use crate::{
    objects::{Objects, Vertex},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Vertex {
    fn transform_with_cache(
        self,
        _: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        Self::new()
    }
}
