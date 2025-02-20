use fj_math::Transform;

use crate::{Core, topology::Face};

use super::{TransformCache, TransformObject};

impl TransformObject for Face {
    type Transformed = Self;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let surface = self
            .surface()
            .clone()
            .transform_with_cache(transform, core, cache);
        let region = (self.region(), self.surface())
            .transform_with_cache(transform, core, cache);

        Self::new(surface, region)
    }
}
