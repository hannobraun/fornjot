use crate::math::Point;

/// # An axis-aligned bounding box
pub struct Aabb<const D: usize> {
    pub min: Point<D>,
    pub max: Point<D>,
}
