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

impl<S, T> From<[S; 2]> for BoundaryOnCurve<T>
where
    S: Into<T>,
{
    fn from(boundary: [S; 2]) -> Self {
        let inner = boundary.map(Into::into);
        Self { inner }
    }
}
