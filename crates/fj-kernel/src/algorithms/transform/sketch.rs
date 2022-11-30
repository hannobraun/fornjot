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
        _: &mut TransformCache,
    ) -> Self {
        let faces = self
            .faces()
            .into_iter()
            .cloned()
            .map(|face| face.transform(transform, objects));
        Sketch::builder().with_faces(faces).build(objects)
    }
}
