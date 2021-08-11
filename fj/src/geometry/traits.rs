use nalgebra::{Point, SVector, Unit};

use crate::geometry::aabb::Aabb;

/// Defines geometry that can be samples
///
/// The `D` parameter defines the dimensionality of the geometry (typically
/// geometry would be 2- or 3-dimensional).
pub trait Geometry<const D: usize> {
    /// Sample the geometry at the specified point
    ///
    /// Returns a `Sample` value which describes, among other attributes, the
    /// distance of the point from the surface.
    fn sample(&self, point: impl Into<Point<f32, D>>) -> Sample<D>;

    // TASK: Add method that calls `sample` multiple times to compute the
    //       surface normal for a specific point.
}

/// The result of sampling geometry at a specific point
///
/// Returned by [`Geometry::sample`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sample<const D: usize> {
    /// The point at which the geometry was sampled
    pub point: Point<f32, D>,

    /// The minimum distance of the point to the surface
    ///
    /// A positive value indicates that the point is outside of the object, a
    /// negative value indicates that the point is inside. Either way, the
    /// absolute value is equal to the distance of the point to the surface.
    pub distance: f32,

    // TASK: Remove normal from `SurfaceSample`.
    //
    //       It can be computed by sampling the signed distance function. This
    //       would require only one piece of code for all shapes, instead of a
    //       specific implementation for all implementations of `SurfaceSample`.
    pub normal: Unit<SVector<f32, D>>,
}

/// Defines a bounding volume that encloses geometry
pub trait BoundingVolume<const D: usize> {
    /// Return the geometry's axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}
