use nalgebra::vector;

use crate::math::{Point, Vector};

/// Shapes that describe a path
///
/// `D` defines the dimension that the path is described in.
pub trait Path<const D: usize> {
    /// Return the vector that describes the path
    ///
    /// Only straight paths are supported right now.
    fn path(&self) -> Vector<D>;

    /// Compute a point along the path
    ///
    /// `s` defines which point along the path should be computed. Typically, a
    /// value of `0.0` defines the beginning of the path, `1.0` its end.
    /// Implementations are allowed to return points for other values, too.
    fn point_at(&self, _s: f32) -> Point<D> {
        // TASK: Remove this default implementation.
        todo!()
    }

    /// Compute the next point along the path
    ///
    /// Returns a float value that describes the point along the path, as well
    /// as the point itself.
    ///
    /// Given a point `p1`, described by `from_s`, and a point `p2` returned by
    /// this function, the following must hold for any point `p` on the path
    /// between `p1` and `p2`:
    ///
    /// The distance of `p` to the line segment whose end points are `p1` and
    /// `p2` must by equal to or smaller than `tolerance`.
    fn next_point(&self, _from_s: f32, _tolerance: f32) -> (f32, Point<D>) {
        // TASK: Remove this default implementation.
        todo!()
    }
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
