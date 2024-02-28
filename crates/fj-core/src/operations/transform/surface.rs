use fj_math::Transform;

use crate::{
    objects::Surface, operations::insert::Insert, storage::Handle, Core,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Handle<Surface> {
    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self {
        cache
            .entry(self)
            .or_insert_with(|| {
                let geometry = self.geometry().transform(transform);
                Surface::new(geometry).insert(core)
            })
            .clone()
    }
}
