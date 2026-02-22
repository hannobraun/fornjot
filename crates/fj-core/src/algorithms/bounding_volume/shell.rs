use crate::{geometry::Geometry, math::Aabb, topology::Shell};

impl super::BoundingVolume<3> for &Shell {
    fn aabb(self, geometry: &Geometry) -> Option<Aabb<3>> {
        let mut aabb: Option<Aabb<3>> = None;

        for face in self.faces() {
            let new_aabb = face.aabb(geometry);
            aabb = aabb.map_or(new_aabb, |aabb| match new_aabb {
                Some(new_aabb) => Some(aabb.merged(&new_aabb)),
                None => Some(aabb),
            });
        }

        aabb
    }
}
