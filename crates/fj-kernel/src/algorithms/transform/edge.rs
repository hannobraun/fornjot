use fj_math::Transform;

use crate::{
    objects::Objects,
    partial::{PartialGlobalEdge, PartialHalfEdge},
    services::Service,
};

use super::TransformObject;

impl TransformObject for PartialHalfEdge {
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
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
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let curve = self.curve.transform(transform, objects);
        let vertices = self
            .vertices
            .map(|vertex| vertex.transform(transform, objects));

        Self { curve, vertices }
    }
}
