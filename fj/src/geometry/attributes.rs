use nalgebra::{Point, SVector, Unit};

use crate::geometry::aabb::Aabb;

/// Provides a signed distance function
pub trait Geometry<const D: usize> {
    fn sample(&self, point: impl Into<Point<f32, D>>) -> Sample<D>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sample<const D: usize> {
    pub point: Point<f32, D>,
    pub distance: f32,

    // TASK: Remove normal from `SurfaceSample`.
    //
    //       It can be computed by sampling the signed distance function. This
    //       would require only one piece of code for all shapes, instead of a
    //       specific implementation for all implementations of `SurfaceSample`.
    pub normal: Unit<SVector<f32, D>>,
}

pub trait BoundingVolume<const D: usize> {
    /// Axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}
