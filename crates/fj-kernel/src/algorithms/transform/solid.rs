use fj_math::Transform;

use crate::{objects::Solid, services::Services};

use super::{TransformCache, TransformObject};

impl TransformObject for Solid {
    fn transform_with_cache(
        self,
        transform: &Transform,
        services: &mut Services,
        cache: &mut TransformCache,
    ) -> Self {
        let shells = self.shells().cloned().map(|shell| {
            shell.transform_with_cache(transform, services, cache)
        });

        Self::new(shells)
    }
}
