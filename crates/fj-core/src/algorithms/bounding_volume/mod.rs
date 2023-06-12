//! Compute a bounding volume for an object

use fj_math::Aabb;

/// Compute a bounding volume for an object
pub trait BoundingVolume<const D: usize> {
    /// Compute an axis-aligned bounding box (AABB)
    ///
    /// Return `None`, if no AABB can be computed (if the object is empty).
    fn aabb(&self) -> Option<Aabb<D>>;
}
