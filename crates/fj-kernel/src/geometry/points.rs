/// A point that stores a local and a canonical form
///
/// The local form of a point is whatever representation is most appropriate in
/// the current context. The canonical form is the representation that the
/// local form was created from.
///
/// Typically, the canonical form is more general and has higher dimensionality
/// (for example, a point in a 3D space), while the local form is more specific
/// and has lower dimensionality (for example, the point in 2D surface
/// coordinates, on surface within that 3D space).
///
/// The purpose of storing both forms is to be able to losslessly convert the
/// point back to its canonical form. Even if this conversion can be computed on
/// the fly, such a conversion might not result in the original canonical form,
/// due to floating point accuracy issues. Hence, such a conversion would not be
/// lossless, which could result in bugs.
///
/// The `N` parameter defines the dimensionality of the local form, while the
/// `C` parameter defines the dimensionality of the canonical form.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point<const N: usize, const C: usize> {
    native: fj_math::Point<N>,
    canonical: fj_math::Point<C>,
}

impl<const N: usize, const C: usize> Point<N, C> {
    /// Construct a new instance
    ///
    /// Both the local and the canonical form must be provided. The caller must
    /// guarantee that both of them match, i.e. define the same point.
    pub fn new(
        native: impl Into<fj_math::Point<N>>,
        canonical: impl Into<fj_math::Point<C>>,
    ) -> Self {
        Self {
            native: native.into(),
            canonical: canonical.into(),
        }
    }

    /// Access the point's local form
    pub fn local(&self) -> fj_math::Point<N> {
        self.native
    }

    /// Access the point's canonical form
    pub fn canonical(&self) -> fj_math::Point<C> {
        self.canonical
    }
}
