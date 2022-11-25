use fj_math::Transform;

use crate::{
    objects::{Objects, Solid},
    services::Service,
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for Handle<Solid> {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let faces = self
            .shells()
            .cloned()
            .map(|shell| shell.transform(transform, objects));
        Solid::builder().with_shells(faces).build(objects)
    }
}
