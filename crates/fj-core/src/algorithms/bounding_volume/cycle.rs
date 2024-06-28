use fj_math::Aabb;

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{Cycle, Surface},
};

impl super::BoundingVolume<2> for (&Cycle, &Handle<Surface>) {
    fn aabb(self, geometry: &Geometry) -> Option<Aabb<2>> {
        let (cycle, surface) = self;

        let mut aabb: Option<Aabb<2>> = None;

        for (half_edge, half_edge_next) in cycle.half_edges().pairs() {
            let new_aabb = (half_edge, half_edge_next.start_vertex(), surface)
                .aabb(geometry)
                .expect("`HalfEdge` can always compute AABB");
            aabb = Some(aabb.map_or(new_aabb, |aabb| aabb.merged(&new_aabb)));
        }

        aabb
    }
}
