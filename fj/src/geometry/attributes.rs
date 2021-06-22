use nalgebra::Point;

use crate::geometry::aabb::Aabb;

/// Provides a signed distance function
pub trait Surface<const D: usize> {
    // TASK: Return struct that can also include normal.
    fn surface(&self, point: impl Into<Point<f32, D>>) -> f32;
}

pub trait BoundingVolume<const D: usize> {
    /// Axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}
