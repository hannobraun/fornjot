pub mod bounding_volume;
pub mod edges;
pub mod faces;
pub mod shapes;
pub mod vertices;

use self::bounding_volume::Aabb;

/// Implemented by all shapes
pub trait Shape {
    /// Compute the axis-aligned bounding box of a shape
    ///
    /// If a shape is empty, its [`Aabb`]'s `min` and `max` points must be equal
    /// (but are otherwise not specified).
    fn aabb(&self) -> Aabb;
}

impl Shape for fj::Shape {
    fn aabb(&self) -> Aabb {
        match self {
            Self::Shape2d(shape) => shape.aabb(),
            Self::Shape3d(shape) => shape.aabb(),
        }
    }
}

impl Shape for fj::Shape2d {
    fn aabb(&self) -> Aabb {
        match self {
            Self::Circle(shape) => shape.aabb(),
            Self::Difference(shape) => shape.aabb(),
            Self::Square(shape) => shape.aabb(),
        }
    }
}

impl Shape for fj::Shape3d {
    fn aabb(&self) -> Aabb {
        match self {
            Self::Sweep(shape) => shape.aabb(),
        }
    }
}
