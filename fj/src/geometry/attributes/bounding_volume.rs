use crate::geometry::{aabb::Aabb, operations, shapes};

pub trait BoundingVolume<const D: usize> {
    /// Axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}

impl<const D: usize> BoundingVolume<D> for shapes::Hypersphere<D> {
    fn aabb(&self) -> Aabb<D> {
        Aabb {
            min: [-self.radius; D].into(),
            max: [self.radius; D].into(),
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

impl<Sketch> BoundingVolume<3> for operations::LinearExtrude<Sketch>
where
    Sketch: BoundingVolume<2>,
{
    fn aabb(&self) -> Aabb<3> {
        self.sketch
            .aabb()
            .extend(-self.height / 2.0, self.height / 2.0)
    }
}
