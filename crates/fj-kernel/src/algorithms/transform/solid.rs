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
        _: &mut TransformCache,
    ) -> Self {
        let faces = self
            .shells()
            .cloned()
            .map(|shell| shell.transform(transform, objects));
        Solid::builder().with_shells(faces).build(objects)
    }
}
