use fj_math::Transform;

use crate::{
    objects::{Objects, Solid},
    services::Service,
    storage::Handle,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Handle<Solid> {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let faces = self
            .shells()
            .cloned()
            .map(|shell| shell.transform_with_cache(transform, objects, cache));
        Solid::builder().with_shells(faces).build(objects)
    }
}
