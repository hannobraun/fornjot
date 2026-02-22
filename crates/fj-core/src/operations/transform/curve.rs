use crate::{
    Core,
    math::Transform,
    operations::insert::Insert,
    storage::Handle,
    topology::{Curve, Surface},
};

use super::{TransformCache, TransformObject};

impl TransformObject for (&Handle<Curve>, &Handle<Surface>) {
    type Transformed = Handle<Curve>;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let (curve, surface) = self;

        // We don't actually need to transform the curve, as its geometry is
        // locally defined on a surface. We need to transform that surface
        // though, or what we do here will have no consequence.
        //
        // If this transformation is only one element in the transformation of
        // a whole object graph, using the cache here ensures that the surface
        // doesn't get transformed multiple times.
        let transformed_surface = cache
            .entry(surface)
            .or_insert_with(|| surface.transform(transform, core))
            .clone();
        let transformed_curve = cache
            .entry(curve)
            .or_insert_with(|| Curve::new().insert(core))
            .clone();

        core.layers.geometry.define_curve(
            transformed_curve.clone(),
            transformed_surface,
            core.layers
                .geometry
                .of_curve(curve)
                .unwrap()
                .local_on(surface)
                .unwrap()
                .clone(),
        );

        transformed_curve
    }
}
