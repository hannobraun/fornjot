use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use crate::{math::Point, storage::Handle, topology::Vertex};

/// A boundary on a curve
///
/// This struct is generic, because different situations require different
/// representations of a boundary. In some cases, curve coordinates are enough,
/// in other cases, vertices are required, and sometimes you need both.
#[derive(Clone, Copy, Debug)]
pub struct CurveBoundary<T: CurveBoundaryElement> {
    /// The raw representation of the boundary
    pub inner: [T::Repr; 2],
}

impl<T: CurveBoundaryElement> CurveBoundary<T> {
    /// Indicate whether the boundary is normalized
    ///
    /// If the boundary is normalized, its bounding elements are in a defined
    /// order, and calling `normalize` will return an identical instance.
    pub fn is_normalized(&self) -> bool {
        let [a, b] = &self.inner;
        a <= b
    }

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
    /// in a defined order. This can be used to compare boundaries while
    /// disregarding their direction.
    #[must_use]
    pub fn normalize(self) -> Self {
        if self.is_normalized() {
            self
        } else {
            self.reverse()
        }
    }
}

impl Default for CurveBoundary<Point<1>> {
    fn default() -> Self {
        Self {
            inner: [[0.], [1.]].map(Point::from),
        }
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

impl<T: CurveBoundaryElement> Eq for CurveBoundary<T> {}

impl<T: CurveBoundaryElement> PartialEq for CurveBoundary<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: CurveBoundaryElement> Hash for CurveBoundary<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T: CurveBoundaryElement> Ord for CurveBoundary<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T: CurveBoundaryElement> PartialOrd for CurveBoundary<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

impl CurveBoundaryElement for Vertex {
    type Repr = Handle<Vertex>;
}
