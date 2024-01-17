use fj_math::Transform;

use crate::{objects::Face, services::Services};

use super::{TransformCache, TransformObject};

impl TransformObject for Face {
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        let surface = self
            .surface()
            .clone()
            .transform_with_cache(transform, services, cache);
        let region = self
            .region()
            .clone()
            .transform_with_cache(transform, services, cache);

        Self::new(surface, region)
    }
}
