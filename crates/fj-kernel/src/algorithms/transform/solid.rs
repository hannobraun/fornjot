use fj_math::Transform;

use crate::{
    objects::{Objects, Solid},
    storage::Handle,
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for Handle<Solid> {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let faces = self
            .shells()
            .cloned()
            .map(|shell| -> Result<_, ValidationError> {
                shell.transform(transform, objects)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Solid::builder().with_shells(faces).build(objects))
    }
}
