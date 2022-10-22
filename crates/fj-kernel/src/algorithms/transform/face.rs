use fj_math::Transform;

use crate::{
    objects::{Face, Faces, Objects},
    partial::HasPartial,
    storage::Handle,
};

use super::TransformObject;

impl TransformObject for Handle<Face> {
    fn transform(self, transform: &Transform, objects: &Objects) -> Self {
        let surface = self.surface().clone().transform(transform, objects);
        let exterior = self
            .exterior()
            .to_partial()
            .transform(transform, objects)
            .with_surface(Some(surface.clone()))
            .build(objects);
        let interiors = self.interiors().map(|cycle| {
            cycle
                .to_partial()
                .transform(transform, objects)
                .with_surface(Some(surface.clone()))
                .build(objects)
        });

        let color = self.color();

        Face::builder(objects)
            .with_exterior(exterior)
            .with_interiors(interiors)
            .with_color(color)
            .build()
    }
}

impl TransformObject for Faces {
    fn transform(self, transform: &Transform, objects: &Objects) -> Self {
        let mut faces = Faces::new();
        faces.extend(
            self.into_iter()
                .map(|face| face.transform(transform, objects)),
        );
        faces
    }
}
