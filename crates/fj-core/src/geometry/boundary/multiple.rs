use fj_math::Point;

use crate::geometry::CurveBoundary;

/// A collection of multiple [`CurveBoundary`] instances
///
/// Has a type parameter, `T`, which can be used to attach a payload to each
/// boundary.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveBoundaries<T: CurveBoundariesPayload = ()> {
    /// The [`CurveBoundary`] instances
    pub inner: Vec<(CurveBoundary<Point<1>>, T)>,
}

impl<T: CurveBoundariesPayload> CurveBoundaries<T> {
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

    /// Reverse each boundary, and their order
    pub fn reverse(&mut self) {
        self.inner.reverse();

        for (boundary, payload) in &mut self.inner {
            *boundary = boundary.reverse();
            payload.reverse();
        }
    }

    /// Reduce `self` to the subset defined by the provided boundary
    pub fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>) {
        for (b, segment) in &mut self.inner {
            *b = b.subset(boundary);
            segment.make_subset(boundary);
        }

        self.inner.retain(|(boundary, _)| !boundary.is_empty());
    }
}

impl<T: CurveBoundariesPayload> Default for CurveBoundaries<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

/// A payload that can be used in [`CurveBoundaries`]
pub trait CurveBoundariesPayload: Clone + Ord {
    /// Reverse the orientation of the payload
    fn reverse(&mut self);

    /// Reduce the payload to the subset defined by the provided boundary
    fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>);
}

impl CurveBoundariesPayload for () {
    fn reverse(&mut self) {}
    fn make_subset(&mut self, _: CurveBoundary<Point<1>>) {}
}
