//! Ray casting

use super::intersect::HorizontalRayToTheRight;

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
