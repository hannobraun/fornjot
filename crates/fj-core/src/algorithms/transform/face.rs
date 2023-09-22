use fj_math::Transform;

use crate::{
    objects::{Face, Region},
    operations::Insert,
    services::Services,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Face {
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        // Color does not need to be transformed.
        let color = self.region().color();

        let surface = self
            .surface()
            .clone()
            .transform_with_cache(transform, services, cache);
        let exterior = self
            .region()
            .exterior()
            .clone()
            .transform_with_cache(transform, services, cache);
        let interiors =
            self.region().interiors().iter().cloned().map(|interior| {
                interior.transform_with_cache(transform, services, cache)
            });

        let region = Region::new(exterior, interiors, color).insert(services);

        Self::new(surface, region)
    }
}
