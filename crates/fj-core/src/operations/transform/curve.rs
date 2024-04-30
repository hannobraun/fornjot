use fj_math::Transform;

use crate::{
    operations::insert::Insert, storage::Handle, topology::Curve, Core,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Handle<Curve> {
    fn transform_with_cache(
        &self,
        _: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self {
        // There's nothing to actually transform here, as `Curve` holds no data.
        // We still need this implementation though, as a new `Curve` object
        // must be created to represent the new and transformed curve.
        cache
            .entry(self)
            .or_insert_with(|| Curve::new().insert(core))
            .clone()
    }
}
