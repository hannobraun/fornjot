use crate::{LineSegment, Point};

/// A polygonal chain
///
/// The dimensionality of the polygonal chain is defined by the const generic
/// `D` parameter.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct PolyChain<const D: usize> {
    points: Vec<Point<D>>,
}

impl<const D: usize> PolyChain<D> {
    /// Create an empty `PolyChain`
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a polygonal chain from a number of points
    pub fn from_points(
        points: impl IntoIterator<Item = impl Into<Point<D>>>,
    ) -> Self {
        let points = points.into_iter().map(Into::into).collect::<Vec<_>>();

        // Validate that we don't have any neighboring points that are the same.
        // This doesn't ensure that the `PolyChain` is fully valid, but it's
        // better than nothing.
        for points in points.windows(2) {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable"
            // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
            let [a, b] = [&points[0], &points[1]];

            assert_ne!(a, b, "Polygonal chain has duplicate point");
        }

        Self { points }
    }

    /// Access the segments of the polygonal chain
    pub fn segments(&self) -> Vec<LineSegment<D>> {
        let mut segments = Vec::new();

        for points in self.points.windows(2) {
            // Can't panic, as we passed `2` to `windows`. Can be cleaned up,
            // once `array_windows` is stable.
            let points = [points[0], points[1]];

            let segment = LineSegment { points };
            segments.push(segment);
        }

        segments
    }

    /// Close the polygonal chain
    ///
    /// Adds the first point of the chain as the last, closing the chain. This
    /// method does not check whether the `PolyChain` is already closed.
    pub fn close(mut self) -> Self {
        if let Some(&point) = self.points.first() {
            self.points.push(point);
        }

        self
    }

    /// Reverse the order of points in the `PolyChain`
    pub fn reverse(mut self) -> Self {
        self.points.reverse();
        self
    }
}

impl<P, Ps, const D: usize> From<Ps> for PolyChain<D>
where
    P: Into<Point<D>>,
    Ps: IntoIterator<Item = P>,
{
    fn from(points: Ps) -> Self {
        Self::from_points(points)
    }
}
