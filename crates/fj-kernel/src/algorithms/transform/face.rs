use fj_math::Transform;

use crate::{
    insert::Insert,
    objects::{Face, FaceSet, Objects},
    partial::{HasPartial, PartialFace},
    services::Service,
};

use super::TransformObject;

impl TransformObject for PartialFace {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let surface = self
            .surface()
            .map(|surface| surface.transform(transform, objects));
        let exterior = self
            .exterior()
            .into_partial()
            .transform(transform, objects)
            .with_surface(surface.clone());
        let interiors = self.interiors().map(|cycle| {
            cycle
                .into_partial()
                .transform(transform, objects)
                .with_surface(surface.clone())
                .build(objects)
                .insert(objects)
        });

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

        face
    }
}

impl TransformObject for FaceSet {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let mut faces = FaceSet::new();
        faces.extend(
            self.into_iter()
                .map(|face| face.transform(transform, objects)),
        );
        faces
    }
}
