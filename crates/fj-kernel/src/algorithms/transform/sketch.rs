use fj_math::Transform;

use crate::{
    objects::{Objects, Sketch},
    services::Service,
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for Handle<Sketch> {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let faces = self
            .faces()
            .into_iter()
            .cloned()
            .map(|face| face.transform(transform, objects));
        Sketch::builder().with_faces(faces).build(objects)
    }
}
