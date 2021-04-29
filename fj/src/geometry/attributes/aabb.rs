use nalgebra::Point;
use parry3d::bounding_volume::AABB;

use crate::geometry::shapes::Circle;

/// Axis-aligned bounding box
pub trait Aabb {
    fn aabb(&self) -> AABB;
}

impl Aabb for Circle {
    fn aabb(&self) -> AABB {
        AABB {
            mins: Point::<_, 3>::new(-self.radius(), -self.radius(), 0.0),
            maxs: Point::<_, 3>::new(self.radius(), self.radius(), 0.0),
        }
    }
}
