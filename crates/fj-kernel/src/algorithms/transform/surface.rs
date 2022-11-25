use fj_math::Transform;

use crate::{
    geometry::surface::SurfaceGeometry, objects::Objects,
    partial::PartialSurface, services::Service,
};

use super::TransformObject;

impl TransformObject for PartialSurface {
    fn transform(
        self,
        transform: &Transform,
        _: &mut Service<Objects>,
    ) -> Self {
        let geometry = self.geometry.map(|geometry| {
            let u = geometry.u.transform(transform);
            let v = transform.transform_vector(&geometry.v);

            SurfaceGeometry { u, v }
        });

        Self { geometry }
    }
}
