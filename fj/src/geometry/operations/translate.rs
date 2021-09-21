use crate::math::Vector;

/// Translates a shape
///
/// `D` defines the dimensionality of the translation. Typically, translations
/// will be 1-, 2-, or 3-dimensional.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Translate<T, const D: usize> {
    /// The shape being translated.
    pub shape: T,

    /// The offset created by the translation
    pub offset: Vector<D>,
}
