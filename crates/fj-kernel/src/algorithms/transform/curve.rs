use fj_math::Transform;

use crate::{
    objects::{Curve, GlobalCurve},
    stores::{Handle, Stores},
};

use super::TransformObject;

impl TransformObject for Curve {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let surface = self.surface().transform(transform, stores);
        let global_form =
            self.global_form().clone().transform(transform, stores);

        // Don't need to transform `self.path`, as that's defined in surface
        // coordinates, and thus transforming `surface` takes care of it.
        Self::new(surface, self.path(), global_form)
    }
}

impl TransformObject for Handle<GlobalCurve> {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        stores.global_curves.insert(GlobalCurve::from_path(
            self.path().transform(transform, stores),
        ))
    }
}
