use crate::{
    geometry::{operations, shapes},
    math::{Point, Vector},
};

/// Implemented for geometry that defines a signed distance field
///
/// The `D` parameter defines the dimensionality of the geometry (typically
/// geometry would be 2- or 3-dimensional).
pub trait SignedDistanceField<const D: usize> {
    /// Compute distance to surface at the specified point
    ///
    /// Returns a `Distance` value which indicates the distance of the point
    /// from the surface.
    fn distance(&self, point: impl Into<Point<D>>) -> Distance<D>;
}

impl<const D: usize> SignedDistanceField<D> for shapes::Hypersphere<D> {
    fn distance(&self, point: impl Into<Point<D>>) -> Distance<D> {
        let point = point.into();

        Distance {
            point,
            distance: point.coords.magnitude() - self.radius,
        }
    }
}

impl<A, B, const D: usize> SignedDistanceField<D>
    for operations::Difference<A, B>
where
    A: SignedDistanceField<D>,
    B: SignedDistanceField<D>,
{
    fn distance(&self, point: impl Into<Point<D>>) -> Distance<D> {
        let point = point.into();

        let dist_a = self.a.distance(point);
        let dist_b = self.b.distance(point);

        let dist_b = Distance {
            distance: -dist_b.distance,
            ..dist_b
        };

        if dist_a.distance > dist_b.distance {
            dist_a
        } else {
            dist_b
        }
    }
}

impl<T, const D: usize> SignedDistanceField<D> for operations::Scale<T>
where
    T: SignedDistanceField<D>,
{
    fn distance(&self, point: impl Into<Point<D>>) -> Distance<D> {
        let mut distance = self.shape.distance(point.into() / self.factor);
        distance.distance *= self.factor;
        distance
    }
}

// TASK: Replace `f32` with `Path`.
impl<T> SignedDistanceField<3> for operations::Sweep<T, f32>
where
    T: SignedDistanceField<2>,
{
    fn distance(&self, point: impl Into<Point<3>>) -> Distance<3> {
        let point = point.into();

        let sample_xy = self.shape.distance(point.xy());

        let d_xy = sample_xy.distance;
        let d_z = if point.z <= 0. {
            point.z.abs()
        } else if point.z > 0. && point.z < self.path {
            -f32::min(point.z, self.path - point.z)
        } else {
            point.z - self.path
        };

        let w = Vector::from([f32::max(d_xy, 0.0), f32::max(d_z, 0.0)]);

        let distance = f32::min(f32::max(d_xy, d_z), 0.0) + w.magnitude();

        Distance { point, distance }
    }
}

/// The minimum distance of a specific point to a surface
///
/// Returned by [`SignedDistanceField::distance`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Distance<const D: usize> {
    /// The point from which the distance was determined
    pub point: Point<D>,

    /// The minimum distance of the point to the surface
    ///
    /// A positive value indicates that the point is outside of the object, a
    /// negative value indicates that the point is inside. Either way, the
    /// absolute value is equal to the distance of the point to the surface.
    pub distance: f32,
}
