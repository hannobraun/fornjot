use fj_math::Transform;

use crate::{
    objects::{Objects, Shell},
    storage::Handle,
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for Handle<Shell> {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let faces = self
            .faces()
            .clone()
            .into_iter()
            .map(|face| -> Result<_, ValidationError> {
                face.transform(transform, objects)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Shell::builder().with_faces(faces).build(objects))
    }
}
