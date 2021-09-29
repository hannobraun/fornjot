use crate::math::Point;

pub struct Triangle<const D: usize>(pub [Point<D>; 3]);

impl<const D: usize> Triangle<D> {
    /// Normalize the triangle
    ///
    /// Given two triangles that contain the same points, calling this method on
    /// them will return the same triangle.
    pub fn normalize(self) -> Self {
        // TASK: Implement.
        todo!()
    }
}
