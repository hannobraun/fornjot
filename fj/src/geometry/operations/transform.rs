use nalgebra::Rotation;

use crate::math::Vector;

/// Transforms (translates and rotates) a shape
///
/// `D` defines the dimensionality of the transformation. Typically,
/// transformations will be 2- or 3-dimensional.
pub struct Transform<T, const D: usize> {
    /// The shape being transformed
    pub shape: T,

    /// The translational part of the transformation
    pub translation: Vector<D>,

    /// The rotational part of the transformation
    pub rotation: Rotation<f32, D>,
}
