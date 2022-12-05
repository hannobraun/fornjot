use fj_math::Transform;

use crate::{
    objects::{Objects, Sketch},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Sketch {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let faces =
            self.faces().into_iter().cloned().map(|face| {
                face.transform_with_cache(transform, objects, cache)
            });

        Self::new(faces)
    }
}
