use std::collections::VecDeque;

use fj_math::Point;

use crate::geometry::CurveBoundary;

/// A collection of multiple [`CurveBoundary`] instances
///
/// Has a type parameter, `T`, which can be used to attach a payload to each
/// boundary.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveBoundaries<T: CurveBoundariesPayload = ()> {
    inner: Vec<(CurveBoundary<Point<1>>, T)>,
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
                // This is what the caller is asking for. Return it!
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

    /// Merge the provided boundary into `self`
    ///
    /// Return the merged boundary and payload.
    pub fn merge(
        &mut self,
        new_boundary: CurveBoundary<Point<1>>,
        new_payload: T,
    ) {
        let mut overlapping_payloads = VecDeque::new();

        let mut i = 0;
        loop {
            let Some((boundary, _)) = self.inner.get(i) else {
                break;
            };

            if boundary.overlaps(&new_boundary) {
                let payload = self.inner.swap_remove(i);
                overlapping_payloads.push_back(payload);
                continue;
            }

            i += 1;
        }

        let mut merged_boundary = new_boundary;
        let mut merged_payload = new_payload;

        for (boundary, payload) in overlapping_payloads {
            assert!(
                merged_boundary.overlaps(&boundary),
                "Shouldn't merge boundaries that don't overlap."
            );

            merged_boundary = merged_boundary.union(boundary);
            merged_payload.merge(&payload, boundary);
        }

        self.inner.push((merged_boundary, merged_payload));
        self.inner.sort();
    }
}

impl<T: CurveBoundariesPayload> Default for CurveBoundaries<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T: CurveBoundariesPayload> FromIterator<(CurveBoundary<Point<1>>, T)>
    for CurveBoundaries<T>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (CurveBoundary<Point<1>>, T)>,
    {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

/// A payload that can be used in [`CurveBoundaries`]
pub trait CurveBoundariesPayload: Clone + Ord {
    /// Reverse the orientation of the payload
    fn reverse(&mut self);

    /// Reduce the payload to the subset defined by the provided boundary
    fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>);

    /// Merge the provided payload
    fn merge(&mut self, other: &Self, other_boundary: CurveBoundary<Point<1>>);
}

impl CurveBoundariesPayload for () {
    fn reverse(&mut self) {}
    fn make_subset(&mut self, _: CurveBoundary<Point<1>>) {}
    fn merge(&mut self, _: &Self, _: CurveBoundary<Point<1>>) {}
}
