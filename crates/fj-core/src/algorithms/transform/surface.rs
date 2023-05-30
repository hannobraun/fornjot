use fj_math::Transform;

use crate::{objects::Surface, services::Services};

use super::{TransformCache, TransformObject};

impl TransformObject for Surface {
    fn transform_with_cache(
        self,
        transform: &Transform,
        _: &mut Services,
        _: &mut TransformCache,
    ) -> Self {
        let geometry = self.geometry().transform(transform);
        Self::new(geometry)
    }
}
