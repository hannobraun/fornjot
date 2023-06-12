use fj_math::Aabb;

use crate::objects::Shell;

impl super::BoundingVolume<3> for Shell {
    fn aabb(&self) -> Option<Aabb<3>> {
        let mut aabb: Option<Aabb<3>> = None;

        for face in self.faces() {
            let new_aabb = face.aabb();
            aabb = aabb.map_or(new_aabb, |aabb| match new_aabb {
                Some(new_aabb) => Some(aabb.merged(&new_aabb)),
                None => Some(aabb),
            });
        }

        aabb
    }
}
