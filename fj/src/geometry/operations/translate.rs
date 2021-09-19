use nalgebra::SVector;

/// Translates a shape
///
/// `D` defines the dimensionality of the translation. Typically, translations
/// will be 1-, 2-, or 3-dimensional.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Translate<Shape, const D: usize> {
    /// The shape being translated.
    pub shape: Shape,

    /// The offset created by the translation
    pub offset: SVector<f32, D>,
}
