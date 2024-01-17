use crate::objects::Region;

use super::TransformObject;

impl TransformObject for Region {
    fn transform_with_cache(
        &self,
        transform: &fj_math::Transform,
        services: &mut crate::services::Services,
        cache: &mut super::TransformCache,
    ) -> Self {
        // Color does not need to be transformed.
        let color = self.color();

        let exterior = self
            .exterior()
            .clone()
            .transform_with_cache(transform, services, cache);
        let interiors = self.interiors().iter().cloned().map(|interior| {
            interior.transform_with_cache(transform, services, cache)
        });

        Region::new(exterior, interiors, color)
    }
}
