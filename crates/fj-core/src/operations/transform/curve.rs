use fj_math::Transform;

use crate::{
    operations::insert::Insert, storage::Handle, topology::Curve, Core,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Handle<Curve> {
    fn transform_with_cache(
        &self,
        _: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self {
        cache
            .entry(self)
            .or_insert_with(|| {
                // We don't actually need to transform the curve, as its
                // geometry is locally defined on a surface. We need to set that
                // geometry for the new object though, that we created here to
                // represent the transformed curve.

                let curve = Curve::new().insert(core);

                let curve_geom = core.layers.geometry.of_curve(self).cloned();
                if let Some(curve_geom) = curve_geom {
                    for (surface, local_definition) in curve_geom.definitions {
                        core.layers.geometry.define_curve(
                            curve.clone(),
                            surface,
                            local_definition,
                        );
                    }
                }

                curve
            })
            .clone()
    }
}
