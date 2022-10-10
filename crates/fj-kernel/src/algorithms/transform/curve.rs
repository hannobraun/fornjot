use fj_math::Transform;

use crate::{
    objects::{GlobalCurve, Objects},
    partial::PartialCurve,
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for Handle<GlobalCurve> {
    fn transform(self, _: &Transform, stores: &Objects) -> Self {
        // `GlobalCurve` doesn't contain any internal geometry. If it did, that
        // would just be redundant with the geometry of other objects, and this
        // other geometry is already being transformed by other implementations
        // of this trait.
        //
        // All we need to do here is create a new `GlobalCurve` instance, to
        // make sure the transformed `GlobalCurve` has a different identity than
        // the original one.
        GlobalCurve::new(stores)
    }
}

impl TransformObject for PartialCurve {
    fn transform(self, transform: &Transform, stores: &Objects) -> Self {
        let surface = self
            .surface
            .map(|surface| surface.transform(transform, stores));
        let global_form = self
            .global_form
            .map(|global_form| global_form.0.transform(transform, stores));

        // Don't need to transform `self.path`, as that's defined in surface
        // coordinates, and thus transforming `surface` takes care of it.
        Self {
            surface,
            path: self.path,
            global_form: global_form.map(Into::into),
        }
    }
}
