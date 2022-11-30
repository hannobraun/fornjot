use fj_math::Transform;

use crate::{
    objects::{Objects, Surface},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Surface {
    fn transform_with_cache(
        self,
        transform: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        let geometry = self.geometry().transform(transform);
        Self::new(geometry)
    }
}
