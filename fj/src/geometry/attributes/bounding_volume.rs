use nalgebra::Point;

use crate::geometry::{operations, shapes};

pub trait BoundingVolume<const D: usize> {
    /// Axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}

pub struct Aabb<const D: usize> {
    pub min: Point<f32, D>,
    pub max: Point<f32, D>,
}

impl BoundingVolume<3> for shapes::Cylinder {
    fn aabb(&self) -> Aabb<3> {
        Aabb {
            min: [-self.radius, -self.radius, -self.height / 2.0].into(),
            max: [self.radius, self.radius, self.height / 2.0].into(),
        }
    }
}

impl<A, B, const D: usize> BoundingVolume<D> for operations::Difference<A, B>
where
    A: BoundingVolume<D>,
{
    fn aabb(&self) -> Aabb<D> {
        // Since `self.b` is subtracted from `self.a`, the bounding volume of
        // the difference is not going to be bigger than that of `self.a`. Just
        // taking the bounding volume from `self.a` is certainly not optimal,
        // but good enough for now.
        self.a.aabb()
    }
}
