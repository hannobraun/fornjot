use fj_math::Point;

use crate::{
    objects::{Cycle, HalfEdge, Surface},
    stores::Stores,
};

/// API for building a [`Cycle`]
///
/// Also see [`Cycle::builder`].
pub struct CycleBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The surface that the [`Cycle`] is defined in
    pub surface: Surface,
}

impl<'a> CycleBuilder<'a> {
    /// Create a polygon from a list of points
    pub fn build_polygon_from_points(
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
                HalfEdge::builder(self.stores, self.surface)
                    .build_line_segment_from_points(points),
            );
        }

        Cycle::new(self.surface, half_edges)
    }
}
