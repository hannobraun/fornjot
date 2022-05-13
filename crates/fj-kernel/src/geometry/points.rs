/// A point that can be losslessly converted into its canonical form
///
/// The canonical form is always the 3D representation. It needs to be provided
/// when constructing the point, along with the point's native form.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point<const N: usize> {
    /// This point's native form
    ///
    /// The native form of the point is its representation in its native
    /// coordinate system. This could be a 1-dimensional curve, 2-dimensional
    /// surface, or 3-dimensional model coordinate system.
    native: fj_math::Point<N>,

    /// The canonical form of the point
    ///
    /// This is always the 3D representation of the point. Since this is always
    /// kept here, unchanged, as the point is converted into other coordinate
    /// systems, it allows for a lossless conversion back into 3D coordinates,
    /// unaffected by floating point accuracy issues.
    canonical: fj_math::Point<3>,
}

impl<const N: usize> Point<N> {
    /// Construct a new instance
    ///
    /// Both the native and the canonical form must be provide. The caller must
    /// guarantee that both of them match.
    pub fn new(
        native: impl Into<fj_math::Point<N>>,
        canonical: impl Into<fj_math::Point<3>>,
    ) -> Self {
        Self {
            native: native.into(),
            canonical: canonical.into(),
        }
    }

    /// Access the point's native form
    pub fn native(&self) -> fj_math::Point<N> {
        self.native
    }

    /// Access the point's canonical form
    pub fn canonical(&self) -> fj_math::Point<3> {
        self.canonical
    }
}
