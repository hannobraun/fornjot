//! Ray casting

mod edge;
mod segment;

pub use self::segment::RaySegmentHit;

use fj_math::Point;

/// Implemented by types that support ray casting
///
/// # Implementation Note
///
/// This is basically a more limited version of [`Intersect`]. It probably makes
/// sense to migrate all of this trait's implementations to [`Intersect`] and
/// remove this trait.
///
/// [`Intersect`]: super::intersect::Intersect
pub trait CastRay<const D: usize> {
    /// The type that describes a hit of the ray on the implementing type
    type Hit;

    /// Cast a ray against `self`
    fn cast_ray(&self, ray: HorizontalRayToTheRight<D>) -> Option<Self::Hit>;
}

/// A horizontal ray that goes to the right
///
/// For in-kernel use, we don't need anything more flexible, and being exactly
/// horizontal simplifies some calculations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HorizontalRayToTheRight<const D: usize> {
    /// The point where the ray originates
    pub origin: Point<D>,
}

impl<P, const D: usize> From<P> for HorizontalRayToTheRight<D>
where
    P: Into<Point<D>>,
{
    fn from(point: P) -> Self {
        Self {
            origin: point.into(),
        }
    }
}
