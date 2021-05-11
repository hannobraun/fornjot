use nalgebra::Point;

use crate::geometry::{operations, shapes};

pub trait BoundingVolume {
    /// Axis-aligned bounding box
    fn aabb(&self) -> Aabb;
}

pub struct Aabb {
    pub min: Point<f32, 3>,
    pub max: Point<f32, 3>,
}

impl BoundingVolume for shapes::Cylinder {
    fn aabb(&self) -> Aabb {
        Aabb {
            min: [-self.radius, -self.radius, -self.height / 2.0].into(),
            max: [self.radius, self.radius, self.height / 2.0].into(),
        }
    }
}

impl<A, B> BoundingVolume for operations::Difference<A, B>
where
    A: BoundingVolume,
{
    fn aabb(&self) -> Aabb {
        // Since `self.b` is subtracted from `self.a`, the bounding volume of
        // the difference is not going to be bigger than that of `self.a`. Just
        // taking the bounding volume from `self.a` is certainly not optimal,
        // but good enough for now.
        self.a.aabb()
    }
}
