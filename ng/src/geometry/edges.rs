use crate::math::Point;

/// Access the edges of a shape
pub trait Edges {
    /// Compute line segments to approximate the shape's edges
    ///
    /// `tolerance` defines how far these line segments are allowed to deviate
    /// from the actual edges of the shape.
    ///
    /// This method presents a weird API right now, as it just returns all the
    /// segments, not distinguishing which edge they approximate. This design is
    /// simple and in line with current use cases, but not expected to last.
    fn segments(&self, tolerance: f32) -> Vec<[Point; 2]>;
}

impl Edges for fj::Shape {
    fn segments(&self, tolerance: f32) -> Vec<[Point; 2]> {
        match self {
            Self::Shape2d(shape) => shape.segments(tolerance),
            Self::Shape3d(shape) => shape.segments(tolerance),
        }
    }
}

impl Edges for fj::Shape2d {
    fn segments(&self, _: f32) -> Vec<[Point; 2]> {
        match self {
            Self::Circle(_) => {
                // TASK: Implement.
                todo!()
            }
            Self::Square(_) => {
                // TASK: Implement.
                todo!()
            }
        }
    }
}

impl Edges for fj::Shape3d {
    fn segments(&self, _: f32) -> Vec<[Point; 2]> {
        match self {
            Self::Sweep(_) => {
                // TASK: Implement.
                todo!()
            }
        }
    }
}
