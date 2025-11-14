use crate::math::Scalar;

/// # A vector
pub struct Vector<const N: usize> {
    pub components: [Scalar; N],
}
