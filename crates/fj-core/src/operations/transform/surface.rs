use fj_math::Transform;

use crate::{objects::Surface, Instance};

use super::{TransformCache, TransformObject};

impl TransformObject for Surface {
    fn transform_with_cache(
        &self,
        transform: &Transform,
        _: &mut Instance,
        _: &mut TransformCache,
    ) -> Self {
        let geometry = self.geometry().transform(transform);
        Self::new(geometry)
    }
}
