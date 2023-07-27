use fj_math::Point;

/// A boundary on a curve
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct BoundaryOnCurve {
    /// The raw representation of the boundary
    pub inner: [Point<1>; 2],
}

impl BoundaryOnCurve {
    /// Reverse the direction of the boundary
    ///
    /// Returns a new instance of this struct, which has its direction reversed.
    pub fn reverse(self) -> Self {
        let [a, b] = self.inner;
        Self { inner: [b, a] }
    }
}

impl<T> From<[T; 2]> for BoundaryOnCurve
where
    T: Into<Point<1>>,
{
    fn from(boundary: [T; 2]) -> Self {
        let inner = boundary.map(Into::into);
        Self { inner }
    }
}
