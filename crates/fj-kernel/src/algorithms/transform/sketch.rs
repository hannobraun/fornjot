use fj_math::Transform;

use crate::{
    objects::{Objects, Sketch},
    storage::Handle,
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for Handle<Sketch> {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let faces = self
            .faces()
            .into_iter()
            .cloned()
            .map(|face| face.transform(transform, objects))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Sketch::builder(objects).with_faces(faces).build(objects))
    }
}
