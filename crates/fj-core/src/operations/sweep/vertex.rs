use fj_math::Vector;

use crate::{
    objects::{Curve, Vertex},
    operations::insert::Insert,
    services::Services,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Vertex> {
    type Swept = (Handle<Curve>, Self);

    fn sweep_with_cache(
        self,
        _: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> Self::Swept {
        let curve = cache
            .curves
            .entry(self.id())
            .or_insert_with(|| Curve::new().insert(services))
            .clone();

        let vertex = cache
            .vertices
            .entry(self.id())
            .or_insert_with(|| Vertex::new().insert(services))
            .clone();

        (curve, vertex)
    }
}
