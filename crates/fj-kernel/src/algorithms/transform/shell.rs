use fj_math::Transform;

use crate::{objects::Shell, services::Services};

use super::{TransformCache, TransformObject};

impl TransformObject for Shell {
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        let faces =
            self.faces().clone().into_iter().map(|face| {
                face.transform_with_cache(transform, services, cache)
            });

        Self::new(faces)
    }
}
