use fj_math::Transform;

use crate::{objects::Surface, Core};

use super::{TransformCache, TransformObject};

impl TransformObject for Surface {
    fn transform_with_cache(
        &self,
        transform: &Transform,
        _: &mut Core,
        _: &mut TransformCache,
    ) -> Self {
        let geometry = self.geometry().transform(transform);
        Self::new(geometry)
    }
}
