use fj_math::Transform;

use crate::{
    objects::{Face, Faces, Stores},
    partial::HasPartial,
};

use super::TransformObject;

impl TransformObject for Face {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let surface = self.surface().clone().transform(transform, stores);
        let exterior = self
            .exterior()
            .to_partial()
            .transform(transform, stores)
            .with_surface(Some(surface.clone()))
            .build(stores);
        let interiors = self.interiors().map(|cycle| {
            cycle
                .to_partial()
                .transform(transform, stores)
                .with_surface(Some(surface.clone()))
                .build(stores)
        });

        let color = self.color();

        Face::from_exterior(exterior)
            .with_interiors(interiors)
            .with_color(color)
    }
}

impl TransformObject for Faces {
    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let mut faces = Faces::new();
        faces.extend(
            self.into_iter()
                .map(|face| face.transform(transform, stores)),
        );
        faces
    }
}
