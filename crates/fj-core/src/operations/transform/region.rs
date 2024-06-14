use crate::{
    operations::insert::Insert,
    storage::Handle,
    topology::{Region, Surface},
    Core,
};

use super::TransformObject;

impl TransformObject for (&Handle<Region>, &Handle<Surface>) {
    type Transformed = Handle<Region>;

    fn transform_with_cache(
        self,
        transform: &fj_math::Transform,
        core: &mut Core,
        cache: &mut super::TransformCache,
    ) -> Self::Transformed {
        let (region, surface) = self;

        let exterior = (region.exterior(), surface)
            .transform_with_cache(transform, core, cache);
        let interiors = region.interiors().iter().map(|interior| {
            (interior, surface).transform_with_cache(transform, core, cache)
        });

        Region::new(exterior, interiors).insert(core)
    }
}
