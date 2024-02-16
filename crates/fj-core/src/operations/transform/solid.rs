use fj_math::Transform;

use crate::{objects::Solid, Core};

use super::{TransformCache, TransformObject};

impl TransformObject for Solid {
    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self {
        let shells =
            self.shells().iter().cloned().map(|shell| {
                shell.transform_with_cache(transform, core, cache)
            });

        Self::new(shells)
    }
}
