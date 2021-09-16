use nalgebra::Point;

/// Implemented for geometry that defines a signed distance field
///
/// The `D` parameter defines the dimensionality of the geometry (typically
/// geometry would be 2- or 3-dimensional).
pub trait SignedDistanceField<const D: usize> {
    /// Compute distance to surface at the specified point
    ///
    /// Returns a `Distance` value which indicates the distance of the point
    /// from the surface.
    fn distance(&self, point: impl Into<Point<f32, D>>) -> Distance<D>;
}

/// The minimum distance of a specific point to a surface
///
/// Returned by [`Geometry::sample`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distance<const D: usize> {
    /// The point from which the distance was determined
    pub point: Point<f32, D>,

    /// The minimum distance of the point to the surface
    ///
    /// A positive value indicates that the point is outside of the object, a
    /// negative value indicates that the point is inside. Either way, the
    /// absolute value is equal to the distance of the point to the surface.
    pub distance: f32,
}
