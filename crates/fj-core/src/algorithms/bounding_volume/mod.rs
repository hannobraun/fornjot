//! Compute a bounding volume for an object

mod cycle;
mod face;
mod half_edge;
mod shell;
mod solid;

use fj_math::Aabb;

use crate::geometry::Geometry;

/// Compute a bounding volume for an object
pub trait BoundingVolume<const D: usize> {
    /// Compute an axis-aligned bounding box (AABB)
    ///
    /// Return `None`, if no AABB can be computed (if the object is empty).
    fn aabb(self, geometry: &Geometry) -> Option<Aabb<D>>;
}
