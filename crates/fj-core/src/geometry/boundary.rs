/// A boundary on a curve
///
/// This struct is generic, because different situations require different
/// representations of a boundary. In some cases, curve coordinates are enough,
/// in other cases, vertices are required, and sometimes you need both.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveBoundary<T> {
    /// The raw representation of the boundary
    pub inner: [T; 2],
}

impl<T> CurveBoundary<T> {
    /// Reverse the direction of the boundary
    ///
    /// Returns a new instance of this struct, which has its direction reversed.
    #[must_use]
    pub fn reverse(self) -> Self {
        let [a, b] = self.inner;
        Self { inner: [b, a] }
    }

    /// Normalize the boundary
    ///
    /// Returns a new instance of this struct, which has the bounding elements
    /// in a defined order. This can be used to compare a boundary while
    /// disregarding its direction.
    #[must_use]
    pub fn normalize(mut self) -> Self
    where
        T: Ord,
    {
        self.inner.sort();
        self
    }
}

impl<S, T> From<[S; 2]> for CurveBoundary<T>
where
    S: Into<T>,
{
    fn from(boundary: [S; 2]) -> Self {
        let inner = boundary.map(Into::into);
        Self { inner }
    }
}
