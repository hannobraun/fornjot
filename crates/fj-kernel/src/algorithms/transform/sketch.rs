use fj_math::Transform;

use crate::{objects::Sketch, services::Services};

use super::{TransformCache, TransformObject};

impl TransformObject for Sketch {
    fn transform_with_cache(
        self,
        _transform: &Transform,
        _services: &mut Services,
        _cache: &mut TransformCache,
    ) -> Self {
        todo!()
    }
}
