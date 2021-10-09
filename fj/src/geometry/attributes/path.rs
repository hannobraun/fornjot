use nalgebra::vector;

use crate::math::Vector;

/// Shapes that describe a path
///
/// `D` defines the dimension that the path is described in.
pub trait Path<const D: usize> {
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
