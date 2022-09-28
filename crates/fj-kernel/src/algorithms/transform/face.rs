use fj_math::Transform;

use crate::{
    objects::{Face, Faces},
    stores::Stores,
};

use super::TransformObject;

impl TransformObject for Face {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let exterior = self.exterior().clone().transform(transform, stores);
        let interiors = self
            .interiors()
            .map(|cycle| cycle.clone().transform(transform, stores));

        let color = self.color();

        Face::from_exterior(exterior)
            .with_interiors(interiors)
            .with_color(color)
    }
}

impl TransformObject for Faces {
    type Transformed = Self;

    fn transform(self, transform: &Transform, stores: &Stores) -> Self {
        let mut faces = Faces::new();
        faces.extend(
            self.into_iter()
                .map(|face| face.transform(transform, stores)),
        );
        faces
    }
}
