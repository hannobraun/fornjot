use fj_math::Transform;

use crate::{
    objects::{Objects, Sketch},
    services::Service,
    storage::Handle,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Handle<Sketch> {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let faces =
            self.faces().into_iter().cloned().map(|face| {
                face.transform_with_cache(transform, objects, cache)
            });
        Sketch::builder().with_faces(faces).build(objects)
    }
}
