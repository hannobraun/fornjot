use fj_math::Aabb;

use crate::{geometry::Geometry, topology::Cycle};

impl super::BoundingVolume<2> for &Cycle {
    fn aabb(self, geometry: &Geometry) -> Option<Aabb<2>> {
        let mut aabb: Option<Aabb<2>> = None;

        for edge in self.half_edges() {
            let new_aabb =
                edge.aabb(geometry).expect("`Edge` can always compute AABB");
            aabb = Some(aabb.map_or(new_aabb, |aabb| aabb.merged(&new_aabb)));
        }

        aabb
    }
}
