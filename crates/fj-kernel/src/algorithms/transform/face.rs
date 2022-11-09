use fj_math::Transform;

use crate::{
    objects::{Face, FaceSet, Objects},
    partial::HasPartial,
    storage::Handle,
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for Handle<Face> {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let surface = self.surface().clone().transform(transform, objects)?;
        let exterior = self
            .exterior()
            .to_partial()
            .transform(transform, objects)?
            .with_surface(Some(surface.clone()))
            .build(objects)?;
        let interiors = self
            .interiors()
            .map(|cycle| -> Result<_, ValidationError> {
                cycle
                    .to_partial()
                    .transform(transform, objects)?
                    .with_surface(Some(surface.clone()))
                    .build(objects)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let color = self.color();

        Face::builder(objects)
            .with_exterior(exterior)
            .with_interiors(interiors)
            .with_color(color)
            .build()
    }
}

impl TransformObject for FaceSet {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let mut faces = FaceSet::new();
        faces.extend(
            self.into_iter()
                .map(|face| -> Result<_, ValidationError> {
                    face.transform(transform, objects)
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
        Ok(faces)
    }
}
