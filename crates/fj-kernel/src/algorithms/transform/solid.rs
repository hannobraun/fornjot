use fj_math::Transform;

use crate::{
    objects::{Objects, Solid},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Solid {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let shells = self
            .shells()
            .cloned()
            .map(|shell| shell.transform_with_cache(transform, objects, cache));

        Self::new(shells)
    }
}
