use fj_math::Transform;

use crate::{objects::Shell, Core};

use super::{TransformCache, TransformObject};

impl TransformObject for Shell {
    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self {
        let faces = self
            .faces()
            .iter()
            .cloned()
            .map(|face| face.transform_with_cache(transform, core, cache));

        Self::new(faces)
    }
}
