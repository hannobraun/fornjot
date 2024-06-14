use fj_math::Transform;

use crate::{topology::Face, Core};

use super::{TransformCache, TransformObject};

impl TransformObject for Face {
    type Transformed = Self;

    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
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
