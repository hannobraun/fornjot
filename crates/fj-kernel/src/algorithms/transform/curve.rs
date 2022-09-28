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
        let global = self.global_form().transform(transform, stores);

        // Don't need to transform `self.path`, as that's in local form.
        Curve::new(surface, self.path(), global)
    }
}

impl TransformObject for GlobalCurve {
    type Transformed = Handle<Self>;

    fn transform(self, transform: &Transform, stores: &Stores) -> Handle<Self> {
        stores.global_curves.insert(GlobalCurve::from_path(
            self.path().transform(transform, stores),
        ))
    }
}
