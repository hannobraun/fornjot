use crate::{objects::Region, Core};

use super::TransformObject;

impl TransformObject for Region {
    fn transform_with_cache(
        &self,
        transform: &fj_math::Transform,
        core: &mut Core,
        cache: &mut super::TransformCache,
    ) -> Self {
        // Color does not need to be transformed.
        let color = self.color();

        let exterior = self
            .exterior()
            .clone()
            .transform_with_cache(transform, core, cache);
        let interiors = self.interiors().iter().cloned().map(|interior| {
            interior.transform_with_cache(transform, core, cache)
        });

        Region::new(exterior, interiors, color)
    }
}
