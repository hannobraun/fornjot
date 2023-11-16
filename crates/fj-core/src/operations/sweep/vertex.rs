use fj_math::Vector;

use crate::{
    objects::{Curve, Vertex},
    operations::insert::Insert,
    services::Services,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Vertex> {
    type Swept = (Handle<Curve>, [Self; 2]);

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

        let a = self.clone();
        let b = cache
            .vertices
            .entry(self.id())
            .or_insert_with(|| Vertex::new().insert(services))
            .clone();
        let vertices = [a, b];

        (curve, vertices)
    }
}
