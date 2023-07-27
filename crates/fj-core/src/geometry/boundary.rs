use std::hash::Hash;

use fj_math::Point;

use crate::{objects::Vertex, storage::HandleWrapper};

/// A boundary on a curve
///
/// This struct is generic, because different situations require different
/// representations of a boundary. In some cases, curve coordinates are enough,
/// in other cases, vertices are required, and sometimes you need both.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveBoundary<T: CurveBoundaryElement> {
    /// The raw representation of the boundary
    pub inner: [T::Repr; 2],
}

impl<T: CurveBoundaryElement> CurveBoundary<T> {
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
    pub fn normalize(mut self) -> Self {
        self.inner.sort();
        self
    }
}

impl<S, T: CurveBoundaryElement> From<[S; 2]> for CurveBoundary<T>
where
    S: Into<T::Repr>,
{
    fn from(boundary: [S; 2]) -> Self {
        let inner = boundary.map(Into::into);
        Self { inner }
    }
}

/// An element of a curve boundary
///
/// Used for the type parameter of [`CurveBoundary`].
pub trait CurveBoundaryElement {
    /// The representation the curve boundary element
    ///
    /// This is the actual data stored in [`CurveBoundary`].
    type Repr: Eq + Hash + Ord;
}

impl CurveBoundaryElement for Point<1> {
    type Repr = Self;
}

impl CurveBoundaryElement for HandleWrapper<Vertex> {
    type Repr = Self;
}
