use fj_math::Transform;

use crate::{topology::Shell, Core};

use super::{TransformCache, TransformObject};

impl TransformObject for Shell {
    type Transformed = Self;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let faces = self
            .faces()
            .iter()
            .cloned()
            .map(|face| face.transform_with_cache(transform, core, cache));

        Self::new(faces)
    }
}
