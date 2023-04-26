use fj_math::Transform;

use crate::{
    objects::{Face, FaceSet},
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
        let color = self.color();

        let surface = self
            .surface()
            .clone()
            .transform_with_cache(transform, services, cache);
        let exterior = self
            .exterior()
            .clone()
            .transform_with_cache(transform, services, cache);
        let interiors = self.interiors().cloned().map(|interior| {
            interior.transform_with_cache(transform, services, cache)
        });

        Self::new(surface, exterior, interiors, color)
    }
}

impl TransformObject for FaceSet {
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        let mut faces = Self::new();
        faces.extend(
            self.into_iter().map(|face| {
                face.transform_with_cache(transform, services, cache)
            }),
        );
        faces
    }
}
