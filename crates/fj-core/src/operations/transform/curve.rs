use fj_math::Transform;

use crate::{objects::Curve, Instance};

use super::{TransformCache, TransformObject};

impl TransformObject for Curve {
    fn transform_with_cache(
        &self,
        _: &Transform,
        _: &mut Instance,
        _: &mut TransformCache,
    ) -> Self {
        // There's nothing to actually transform here, as `Curve` holds no data.
        // We still need this implementation though, as a new `Curve` object
        // must be created to represent the new and transformed curve.
        Self::new()
    }
}
