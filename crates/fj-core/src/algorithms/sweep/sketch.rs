use fj_math::Vector;

use crate::{
    objects::{Face, Sketch, Solid, Surface},
    operations::Insert,
    services::Services,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for (Handle<Sketch>, Handle<Surface>) {
    type Swept = Handle<Solid>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> Self::Swept {
        let (sketch, surface) = self;
        let path = path.into();

        let mut shells = Vec::new();
        for region in sketch.regions() {
            let face =
                Face::new(surface.clone(), region.clone()).insert(services);
            let shell = face.sweep_with_cache(path, cache, services);
            shells.push(shell);
        }

        Solid::new(shells).insert(services)
    }
}
