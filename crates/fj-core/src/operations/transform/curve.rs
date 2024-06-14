use fj_math::Transform;

use crate::{
    operations::{geometry::UpdateCurveGeometry, insert::Insert},
    storage::Handle,
    topology::{Curve, Surface},
    Core,
};

use super::{TransformCache, TransformObject};

impl TransformObject for (&Handle<Curve>, &Handle<Surface>) {
    type Transformed = Handle<Curve>;

    fn transform_with_cache(
        self,
        _: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        let (curve, _) = self;

        cache
            .entry(curve)
            .or_insert_with(|| {
                // We don't actually need to transform the curve, as its
                // geometry is locally defined on a surface. We need to set that
                // geometry for the new object though, that we created here to
                // represent the transformed curve.
                Curve::new()
                    .insert(core)
                    .copy_geometry_from(curve, &mut core.layers.geometry)
            })
            .clone()
    }
}
