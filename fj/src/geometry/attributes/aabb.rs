use parry2d::bounding_volume::AABB;

/// Axis-aligned bounding box
pub trait Aabb {
    fn aabb(&self) -> AABB;
}
