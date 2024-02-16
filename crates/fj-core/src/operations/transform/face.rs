use fj_math::Transform;

use crate::{objects::Face, Core};

use super::{TransformCache, TransformObject};

impl TransformObject for Face {
    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self {
        let surface = self
            .surface()
            .clone()
            .transform_with_cache(transform, core, cache);
        let region = self
            .region()
            .clone()
            .transform_with_cache(transform, core, cache);

        Self::new(surface, region)
    }
}
