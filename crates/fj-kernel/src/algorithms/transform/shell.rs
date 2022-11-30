use fj_math::Transform;

use crate::{
    objects::{Objects, Shell},
    services::Service,
    storage::Handle,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Handle<Shell> {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let faces =
            self.faces().clone().into_iter().map(|face| {
                face.transform_with_cache(transform, objects, cache)
            });
        Shell::builder().with_faces(faces).build(objects)
    }
}
