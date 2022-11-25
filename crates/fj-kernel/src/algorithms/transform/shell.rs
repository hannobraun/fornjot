use fj_math::Transform;

use crate::{
    objects::{Objects, Shell},
    services::Service,
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for Handle<Shell> {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let faces = self
            .faces()
            .clone()
            .into_iter()
            .map(|face| face.transform(transform, objects));
        Shell::builder().with_faces(faces).build(objects)
    }
}
