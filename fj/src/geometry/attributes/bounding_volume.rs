use parry3d::bounding_volume::AABB;

use crate::geometry::shapes::Cylinder;

/// Axis-aligned bounding box
pub trait BoundingVolume {
    fn aabb(&self) -> AABB;
}

impl BoundingVolume for Cylinder {
    fn aabb(&self) -> AABB {
        AABB {
            mins: [-self.radius, -self.radius, -self.height / 2.0].into(),
            maxs: [self.radius, self.radius, self.height / 2.0].into(),
        }
    }
}
