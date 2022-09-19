use fj_math::Point;

use crate::{
    objects::{Cycle, HalfEdge, Surface},
    stores::Stores,
};

/// API for building a [`Cycle`]
pub struct CycleBuilder<'a> {
    stores: &'a Stores,
    surface: Surface,
}

impl<'a> CycleBuilder<'a> {
    /// Construct an instance of `CycleBuilder`
    ///
    /// Also see [`Cycle::build`].
    pub fn new(stores: &'a Stores, surface: Surface) -> Self {
        Self { stores, surface }
    }

    /// Create a polygon from a list of points
    pub fn polygon_from_points(
        &self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Cycle {
        let mut points: Vec<_> = points.into_iter().map(Into::into).collect();

        // A polygon is closed, so we need to add the first point at the end
        // again, for the next step.
        if let Some(point) = points.first().cloned() {
            points.push(point);
        }

        let mut half_edges = Vec::new();
        for points in points.windows(2) {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable.
            let points = [points[0], points[1]];

            half_edges.push(
                HalfEdge::build(self.stores, self.surface)
                    .line_segment_from_points(points),
            );
        }

        Cycle::new(self.surface, half_edges)
    }
}
