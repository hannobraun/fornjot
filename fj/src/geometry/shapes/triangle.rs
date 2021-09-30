use crate::math::Point;

pub struct Triangle<const D: usize>(pub [Point<D>; 3]);

impl<const D: usize> Triangle<D> {
    /// Create a new `Triangle`
    pub fn new(points: [Point<D>; 3]) -> Self {
        // TASK: Make sure that points form a triangle.
        // TASK: Normalize triangle.
        Self(points)
    }

    /// Normalize the triangle
    ///
    /// Given two triangles that contain the same points, calling this method on
    /// them will return the same triangle.
    pub fn normalize(self) -> Self {
        // TASK: Implement.
        todo!()
    }
}
