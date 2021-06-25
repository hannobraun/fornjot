use nalgebra::{Point, SVector, Unit};

use crate::geometry::aabb::Aabb;

/// Provides a signed distance function
pub trait Surface<const D: usize> {
    fn sample(&self, point: impl Into<Point<f32, D>>) -> SurfaceSample<D>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceSample<const D: usize> {
    pub point: Point<f32, D>,
    pub distance: f32,
    pub normal: Unit<SVector<f32, D>>,
}

pub trait BoundingVolume<const D: usize> {
    /// Axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}
