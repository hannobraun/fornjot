use parry3d::bounding_volume::AABB;

use crate::geometry::{operations, shapes};

pub trait BoundingVolume {
    /// Axis-aligned bounding box
    fn aabb(&self) -> AABB;
}

impl BoundingVolume for shapes::Cylinder {
    fn aabb(&self) -> AABB {
        AABB {
            mins: [-self.radius, -self.radius, -self.height / 2.0].into(),
            maxs: [self.radius, self.radius, self.height / 2.0].into(),
        }
    }
}

impl<A, B> BoundingVolume for operations::Difference<A, B>
where
    A: BoundingVolume,
{
    fn aabb(&self) -> AABB {
        // Since `self.b` is subtracted from `self.a`, the bounding volume of
        // the difference is not going to be bigger than that of `self.a`. Just
        // taking the bounding volume from `self.a` is certainly not optimal,
        // but good enough for now.
        self.a.aabb()
    }
}
