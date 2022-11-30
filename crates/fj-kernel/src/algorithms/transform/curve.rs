use fj_math::Transform;

use crate::{
    objects::Objects,
    partial::{PartialCurve, PartialGlobalCurve},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for PartialGlobalCurve {
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

impl TransformObject for PartialCurve {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        let surface = self
            .surface
            .map(|surface| surface.transform(transform, objects));
        let global_form = self.global_form.transform(transform, objects);

        // Don't need to transform `self.path`, as that's defined in surface
        // coordinates, and thus transforming `surface` takes care of it.
        PartialCurve {
            path: self.path,
            surface,
            global_form,
        }
    }
}
