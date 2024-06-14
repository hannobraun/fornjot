use crate::{topology::Region, Core};

use super::TransformObject;

impl TransformObject for Region {
    type Transformed = Self;

    fn transform_with_cache(
        self,
        transform: &fj_math::Transform,
        core: &mut Core,
        cache: &mut super::TransformCache,
    ) -> Self::Transformed {
        let region = self;

        let exterior = region
            .exterior()
            .clone()
            .transform_with_cache(transform, core, cache);
        let interiors = region.interiors().iter().cloned().map(|interior| {
            interior.transform_with_cache(transform, core, cache)
        });

        Region::new(exterior, interiors)
    }
}
