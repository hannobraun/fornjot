use fj_math::Point;

use crate::geometry::CurveBoundary;

use super::single::OneOrTwoBoundaries;

/// A collection of multiple [`CurveBoundary`] instances
///
/// Has a type parameter, `T`, which can be used to attach a payload to each
/// boundary.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveBoundaries<T: CurveBoundariesPayload = ()> {
    inner: Vec<(CurveBoundary<Point<1>>, T)>,
}

impl<T: CurveBoundariesPayload> CurveBoundaries<T> {
    /// Create an empty instance of `CurveBoundaries`
    pub fn empty() -> Self {
        Self { inner: Vec::new() }
    }

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
            *b = b.intersection(boundary);
            segment.make_subset(boundary);
        }

        self.inner.retain(|(boundary, _)| !boundary.is_empty());
    }

    /// Create the union between this an another `CurveBoundaries` instance
    pub fn union(mut self, other: impl Into<Self>) -> Self {
        for (other_boundary, other_payload) in other.into().inner {
            let mut overlapping_payloads = Vec::new();

            let mut i = 0;
            loop {
                let Some((boundary, _)) = self.inner.get(i) else {
                    break;
                };

                if boundary.overlaps(&other_boundary) {
                    let payload = self.inner.swap_remove(i);
                    overlapping_payloads.push(payload);
                    continue;
                }

                i += 1;
            }

            let mut merged_boundary = other_boundary;
            let mut merged_payload = other_payload;

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

        self
    }
}

impl CurveBoundaries<()> {
    /// Compute the difference between this instance and another one
    ///
    /// # Implementation Note
    ///
    /// This method is only available for `CurveBoundaries` instances without
    /// payloads, simply because more wasn't needed so far. Support for payloads
    /// can be added by expanding [`CurveBoundariesPayload`] accordingly, and
    /// integrating the new method here.
    pub fn difference(mut self, other: impl Into<Self>) -> Self {
        for (other_boundary, ()) in other.into().inner {
            let mut i = 0;

            loop {
                if i == self.inner.len() {
                    break;
                }

                let (boundary, ()) = self.inner.remove(i);

                match boundary.difference(other_boundary) {
                    Some(OneOrTwoBoundaries::One(b)) => {
                        self.inner.insert(i, (b, ()));
                        i += 1;
                    }
                    Some(OneOrTwoBoundaries::Two([b1, b2])) => {
                        self.inner.insert(i, (b1, ()));
                        i += 1;

                        self.inner.insert(i, (b2, ()));
                        i += 1;
                    }
                    None => {
                        // Nothing to do!
                        //
                        // We already removed the original boundary above, and
                        // if the difference leaves no result, we don't need to
                        // add anything back.
                        //
                        // Don't need to update `i` either. Thanks to the
                        // removal, it already points to the next item.
                    }
                }
            }
        }

        self
    }

    /// Compute the symmetric difference between this instance and another one
    ///
    /// # Implementation Note
    ///
    /// This method is only available for `CurveBoundaries` instances without
    /// payloads, simply because more wasn't needed so far. Support for payloads
    /// can be added by expanding [`CurveBoundariesPayload`] accordingly, and
    /// integrating the new method here.
    pub fn symmetric_difference(self, other: impl Into<Self>) -> Self {
        let other = other.into();
        self.clone()
            .difference(other.clone())
            .union(other.difference(self))
    }
}

impl<T: CurveBoundariesPayload> Default for CurveBoundaries<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T: CurveBoundariesPayload> From<(CurveBoundary<Point<1>>, T)>
    for CurveBoundaries<T>
{
    fn from((boundary, payload): (CurveBoundary<Point<1>>, T)) -> Self {
        Self {
            inner: vec![(boundary, payload)],
        }
    }
}

impl<B> FromIterator<B> for CurveBoundaries<()>
where
    B: Into<CurveBoundary<Point<1>>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = B>,
    {
        iter.into_iter()
            .map(|boundary| (boundary.into(), ()))
            .collect()
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

#[cfg(test)]
mod tests {
    use super::CurveBoundaries;

    #[test]
    fn difference() {
        // There are already extensive tests for `CurveBoundary::difference`,
        // and we don't need to repeat those here. The following tests just make
        // sure that all of the possible return values of that method are
        // handled correctly.

        // Difference results in one boundary.
        diff([[0., 2.]], [[1., 3.]], [[0., 1.]]);

        // Difference results in two boundaries.
        diff([[0., 3.]], [[1., 2.]], [[0., 1.], [2., 3.]]);

        // Difference results in no boundaries.
        diff([[1., 2.]], [[0., 3.]], []);

        // And a combined one, to make sure that everything works with multiple
        // boundaries in the inputs.
        diff(
            [[0., 2.], [4., 7.], [9., 10.]],
            [[1., 3.], [5., 6.], [8., 11.]],
            [[0., 1.], [4., 5.], [6., 7.]],
        );

        fn diff<const A: usize, const B: usize, const X: usize>(
            a: [[f64; 2]; A],
            b: [[f64; 2]; B],
            x: [[f64; 2]; X],
        ) {
            let a = a.map(|boundary| boundary.map(|v| [v]));
            let b = b.map(|boundary| boundary.map(|v| [v]));
            let x = x.map(|boundary| boundary.map(|v| [v]));

            let a = CurveBoundaries::from_iter(a);
            let b = CurveBoundaries::from_iter(b);
            let x = CurveBoundaries::from_iter(x);

            assert_eq!(a.difference(b), x);
        }
    }

    #[test]
    fn union() {
        union([[0., 1.]], [[1., 2.]], [[0., 2.]]);
        union([[0., 1.]], [[2., 3.]], [[0., 1.], [2., 3.]]);
        union([[0., 1.], [2., 3.]], [[1., 2.], [3., 4.]], [[0., 4.]]);
        union(
            [[0., 1.], [2., 3.]],
            [[1., 2.], [4., 5.]],
            [[0., 3.], [4., 5.]],
        );

        fn union<const A: usize, const B: usize, const X: usize>(
            a: [[f64; 2]; A],
            b: [[f64; 2]; B],
            x: [[f64; 2]; X],
        ) {
            let a = a.map(|boundary| boundary.map(|v| [v]));
            let b = b.map(|boundary| boundary.map(|v| [v]));
            let x = x.map(|boundary| boundary.map(|v| [v]));

            let a = CurveBoundaries::from_iter(a);
            let b = CurveBoundaries::from_iter(b);
            let x = CurveBoundaries::from_iter(x);

            assert_eq!(a.union(b), x);
        }
    }
}
