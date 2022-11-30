use fj_math::Transform;

use crate::{
    objects::Objects,
    partial::{PartialGlobalEdge, PartialHalfEdge},
    services::Service,
};

use super::{TransformCache, TransformObject};

impl TransformObject for PartialHalfEdge {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        let curve = self.curve.transform(transform, objects);
        let vertices = self
            .vertices
            .map(|vertex| vertex.transform(transform, objects));
        let global_form = self.global_form.transform(transform, objects);

        Self {
            curve,
            vertices,
            global_form,
        }
    }
}

impl TransformObject for PartialGlobalEdge {
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        _: &mut TransformCache,
    ) -> Self {
        let curve = self.curve.transform(transform, objects);
        let vertices = self
            .vertices
            .map(|vertex| vertex.transform(transform, objects));

        Self { curve, vertices }
    }
}
