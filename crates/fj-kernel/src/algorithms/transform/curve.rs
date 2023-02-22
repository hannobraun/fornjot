use fj_math::Transform;

use crate::{
    objects::{Curve, Objects},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Curve {
    fn transform_with_cache(
        self,
        _: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        // Don't need to transform path, as that's defined in surface
        // coordinates, and thus transforming `surface` takes care of it.
        let path = self.path();

        Self::new(path)
    }
}
