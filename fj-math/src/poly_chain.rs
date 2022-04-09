use crate::{Point, Segment};

/// A polygonal chain
///
/// The dimensionality of the polygonal chain is defined by the const generic
/// `D` parameter.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct PolyChain<const D: usize> {
    points: Vec<Point<D>>,
}

impl<const D: usize> PolyChain<D> {
    /// Create an empty `PolyChain`
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Construct a polygonal chain from a number of points
    pub fn from_points(
        points: impl IntoIterator<Item = impl Into<Point<D>>>,
    ) -> Self {
        let points = points.into_iter().map(Into::into).collect();
        Self { points }
    }

    /// Access the segments of the polygonal chain
    pub fn segments(&self) -> Vec<Segment<D>> {
        let mut segments = Vec::new();

        for points in self.points.windows(2) {
            // Can't panic, as we passed `2` to `windows`. Can be cleaned up,
            // once `array_windows` is stable.
            let points = [points[0], points[1]];

            let segment = Segment::from_points(points);
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
}

impl<const D: usize> Default for PolyChain<D> {
    fn default() -> Self {
        Self::new()
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
