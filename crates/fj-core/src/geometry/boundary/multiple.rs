use fj_math::Point;

use crate::geometry::CurveBoundary;

/// A collection of multiple [`CurveBoundary`] instances
///
/// Has a type parameter, `T`, which can be used to attach a payload to each
/// boundary.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveBoundaries<T = ()> {
    /// The [`CurveBoundary`] instances
    pub inner: Vec<(CurveBoundary<Point<1>>, T)>,
}

impl<T> CurveBoundaries<T> {
    /// Transform `self` into the payload of the single boundary requested
    ///
    /// If there are no boundaries or multiple boundaries in `self`, or if the
    /// one available boundary is not equal to the one requested, return `None`.
    pub fn into_single_payload(
        mut self,
        boundary: CurveBoundary<Point<1>>,
    ) -> Option<T> {
        match self.inner.pop() {
            Some((b, payload)) if self.inner.is_empty() && b == boundary => {
                // We just removed a single element, there are no others, and
                // the removed element's boundary matches the boundary provided
                // to us.
                //
                // This is what the caller was asking for. Return it!
                Some(payload)
            }
            _ => {
                // Either we don't have any elements in here, or we have more
                // than one (which implies there are gaps between them), or we
                // have a single one that doesn't cover the full boundary we
                // were asked for.
                //
                // Either way, we don't have what the caller wants.
                None
            }
        }
    }
}

impl<T> Default for CurveBoundaries<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}
