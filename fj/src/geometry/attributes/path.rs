use crate::math::Vector;

/// Shapes that describe a path
///
/// `D` defines the dimension that the path is described in.
pub trait Path<const D: usize> {
    fn path(&self) -> Vector<D>;
}
