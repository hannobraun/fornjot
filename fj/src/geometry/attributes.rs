use nalgebra::Point;

use crate::geometry::aabb::Aabb;

/// Provides a signed distance function
pub trait Surface<const D: usize> {
    fn surface(&self, point: impl Into<Point<f32, D>>) -> SurfaceSample;
}

pub struct SurfaceSample {
    pub distance: f32,
    // TASK: Add normal.
}

pub trait BoundingVolume<const D: usize> {
    /// Axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}
