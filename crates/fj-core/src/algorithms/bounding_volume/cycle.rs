use fj_math::Aabb;

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{Cycle, Surface},
};

impl super::BoundingVolume<2> for (&Cycle, &Handle<Surface>) {
    fn aabb(self, geometry: &Geometry) -> Option<Aabb<2>> {
        let (cycle, _) = self;

        let mut aabb: Option<Aabb<2>> = None;

        for half_edge in cycle.half_edges() {
            let new_aabb = half_edge
                .aabb(geometry)
                .expect("`HalfEdge` can always compute AABB");
            aabb = Some(aabb.map_or(new_aabb, |aabb| aabb.merged(&new_aabb)));
        }

        aabb
    }
}
