use fj_math::Point;

/// A boundary on a curve
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct BoundaryOnCurve<T> {
    /// The raw representation of the boundary
    pub inner: [T; 2],
}

impl<T> BoundaryOnCurve<T> {
    /// Reverse the direction of the boundary
    ///
    /// Returns a new instance of this struct, which has its direction reversed.
    #[must_use]
    pub fn reverse(self) -> Self {
        let [a, b] = self.inner;
        Self { inner: [b, a] }
    }
}

impl<T> From<[T; 2]> for BoundaryOnCurve<Point<1>>
where
    T: Into<Point<1>>,
{
    fn from(boundary: [T; 2]) -> Self {
        let inner = boundary.map(Into::into);
        Self { inner }
    }
}
