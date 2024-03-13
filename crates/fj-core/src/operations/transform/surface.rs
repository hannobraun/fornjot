use fj_math::Transform;

use crate::{
    objects::Surface, operations::insert::Insert, storage::Handle, Core,
};

use super::{TransformCache, TransformObject};

impl TransformObject for Handle<Surface> {
    fn transform_with_cache(
        &self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self {
        cache
            .entry(self)
            .or_insert_with(|| {
                let geometry =
                    core.layers.geometry.of_surface(self).transform(transform);
                let surface = Surface::new(geometry).insert(core);

                core.layers
                    .geometry
                    .define_surface(surface.clone(), geometry);

                surface
            })
            .clone()
    }
}
