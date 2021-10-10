use nalgebra::vector;

use crate::math::Vector;

/// Shapes that describe a path
///
/// `D` defines the dimension that the path is described in.
pub trait Path<const D: usize> {
    // TASK: Extend this interface, so it can represent arbitrary curved paths.
    //       Idea:
    //       - `fn point_at(&self, s: f32) -> Point<D>; // 0.0 <= s <= 1.0`
    //       - `fn next_s(&self, tolerance: f32) -> f32;`

    /// Return the vector that describes the path
    ///
    /// Only straight paths are supported right now.
    fn path(&self) -> Vector<D>;
}

impl Path<1> for Vector<1> {
    fn path(&self) -> Vector<1> {
        *self
    }
}

impl Path<2> for Vector<1> {
    fn path(&self) -> Vector<2> {
        vector![self.x, 0.0]
    }
}

impl Path<3> for Vector<1> {
    fn path(&self) -> Vector<3> {
        vector![self.x, 0.0, 0.0]
    }
}

impl Path<2> for Vector<2> {
    fn path(&self) -> Vector<2> {
        *self
    }
}

impl Path<3> for Vector<2> {
    fn path(&self) -> Vector<3> {
        vector![self.x, self.y, 0.0]
    }
}

impl Path<3> for Vector<3> {
    fn path(&self) -> Vector<3> {
        *self
    }
}
