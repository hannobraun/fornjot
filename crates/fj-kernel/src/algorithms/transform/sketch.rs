use fj_math::Transform;

use crate::{objects::Sketch, services::Services};

use super::{TransformCache, TransformObject};

impl TransformObject for Sketch {
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        let faces =
            self.faces().into_iter().cloned().map(|face| {
                face.transform_with_cache(transform, services, cache)
            });

        Self::new(faces)
    }
}
