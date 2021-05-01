use nalgebra::Point;
use parry3d::bounding_volume::AABB;

use crate::geometry::shapes::{Circle, Cylinder};

/// Axis-aligned bounding box
pub trait BoundingVolume {
    fn aabb(&self) -> AABB;
}

impl BoundingVolume for Circle {
    fn aabb(&self) -> AABB {
        AABB {
            mins: Point::<_, 3>::new(-self.radius(), -self.radius(), 0.0),
            maxs: Point::<_, 3>::new(self.radius(), self.radius(), 0.0),
        }
    }
}

impl BoundingVolume for Cylinder {
    fn aabb(&self) -> AABB {
        AABB {
            mins: [-self.radius, -self.radius, -self.height / 2.0].into(),
            maxs: [self.radius, self.radius, self.height / 2.0].into(),
        }
    }
}
