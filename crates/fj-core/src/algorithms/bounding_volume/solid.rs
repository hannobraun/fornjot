use fj_math::Aabb;

use crate::objects::Solid;

impl super::BoundingVolume<3> for Solid {
    fn aabb(&self) -> Option<Aabb<3>> {
        let mut aabb: Option<Aabb<3>> = None;

        for shell in self.shells() {
            let new_aabb = shell.aabb();
            aabb = aabb.map_or(new_aabb, |aabb| match new_aabb {
                Some(new_aabb) => Some(aabb.merged(&new_aabb)),
                None => Some(aabb),
            });
        }

        aabb
    }
}
