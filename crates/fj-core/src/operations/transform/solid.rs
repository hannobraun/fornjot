use crate::{Core, math::Transform, topology::Solid};

use super::{TransformCache, TransformObject};

impl TransformObject for Solid {
    type Transformed = Self;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let shells =
            self.shells().iter().cloned().map(|shell| {
                shell.transform_with_cache(transform, core, cache)
            });

        Self::new(shells)
    }
}
