use nalgebra::Rotation;

/// Rotates a shape
///
/// `D` defines the dimensionality of the rotation. Typically, rotations will be
/// 1-, 2-, or 3-dimensional.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotate<T, const D: usize> {
    /// The shape being rotated
    pub shape: T,

    /// The rotation
    pub rotation: Rotation<f32, D>,
}
