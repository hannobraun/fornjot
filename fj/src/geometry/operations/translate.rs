use nalgebra::SVector;

/// Translates a shape
///
/// `D` defines the dimensionality of the translation. Typically, translations
/// will be 1-, 2-, or 3-dimensional.
pub struct Translate<const D: usize> {
    /// The offset created by the translation
    pub offset: SVector<f32, D>,
}
