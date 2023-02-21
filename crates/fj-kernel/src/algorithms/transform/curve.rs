use fj_math::Transform;

use crate::{
    objects::{Curve, GlobalCurve, Objects},
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

impl TransformObject for GlobalCurve {
    fn transform_with_cache(
        self,
        _: &Transform,
        _: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        // `GlobalCurve` doesn't contain any internal geometry. If it did, that
        // would just be redundant with the geometry of other objects, and this
        // other geometry is already being transformed by other implementations
        // of this trait.
        self
    }
}
