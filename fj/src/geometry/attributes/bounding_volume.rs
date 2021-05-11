use crate::geometry::{aabb::Aabb, operations, shapes};

pub trait BoundingVolume<const D: usize> {
    /// Axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}

impl BoundingVolume<2> for shapes::Circle {
    fn aabb(&self) -> Aabb<2> {
        Aabb {
            min: [-self.radius, -self.radius].into(),
            max: [self.radius, self.radius].into(),
        }
    }
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

impl<T> BoundingVolume<3> for operations::LinearExtrude<T>
where
    T: BoundingVolume<2>,
{
    fn aabb(&self) -> Aabb<3> {
        self.sketch
            .aabb()
            .extend(-self.height / 2.0, self.height / 2.0)
    }
}
