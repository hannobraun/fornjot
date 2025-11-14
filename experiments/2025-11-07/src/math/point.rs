use crate::math::Vector;

/// # A point
pub struct Point<const N: usize> {
    pub coordinates: Vector<N>,
}
