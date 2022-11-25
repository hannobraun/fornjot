use fj_math::Transform;

use crate::{
    objects::{Objects, Sketch},
    services::Service,
    storage::Handle,
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for Handle<Sketch> {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Result<Self, ValidationError> {
        let faces = self
            .faces()
            .into_iter()
            .cloned()
            .map(|face| face.transform(transform, objects))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Sketch::builder().with_faces(faces).build(objects))
    }
}
