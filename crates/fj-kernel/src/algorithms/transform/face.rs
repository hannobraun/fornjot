use fj_math::Transform;

use crate::{
    insert::Insert,
    objects::{Face, FaceSet, Objects},
    partial::{HasPartial, PartialFace},
    services::Service,
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for PartialFace {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Result<Self, ValidationError> {
        let surface = self
            .surface()
            .map(|surface| surface.transform(transform, objects))
            .transpose()?;
        let exterior = self
            .exterior()
            .into_partial()
            .transform(transform, objects)?
            .with_surface(surface.clone());
        let interiors = self
            .interiors()
            .map(|cycle| -> Result<_, ValidationError> {
                Ok(cycle
                    .into_partial()
                    .transform(transform, objects)?
                    .with_surface(surface.clone())
                    .build(objects)
                    .insert(objects))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let color = self.color();

        let mut face = Face::partial()
            .with_exterior(exterior)
            .with_interiors(interiors);
        if let Some(surface) = surface {
            face = face.with_surface(surface);
        }
        if let Some(color) = color {
            face = face.with_color(color);
        }

        Ok(face)
    }
}

impl TransformObject for FaceSet {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
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
