use crate::math::Point;

pub struct Triangle<const D: usize>([Point<D>; 3]);

impl<const D: usize> Triangle<D> {
    /// Create a new `Triangle`
    pub fn new(points: [Point<D>; 3]) -> Option<Self> {
        // TASK: Make sure that points form a triangle.
        // TASK: Normalize triangle.
        Some(Self(points))
    }

    /// Return the points of the triangle
    pub fn points(&self) -> [Point<D>; 3] {
        self.0
    }
}
