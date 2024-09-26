//! # Geometry code specific to lines

use fj_math::{Line, LineSegment, Point};

use crate::geometry::{traits::GenPolyline, CurveBoundary, Tolerance};

impl<const D: usize> GenPolyline<D> for Line<D> {
    fn origin(&self) -> Point<D> {
        self.origin()
    }

    fn line_segment_at(
        &self,
        point_curve: Point<1>,
        _: Tolerance,
    ) -> LineSegment<D> {
        // Collapse line segment into a point, as per documentation.
        let point = self.origin() + self.direction() * point_curve.t;

        LineSegment {
            points: [point, point],
        }
    }

    fn generate_polyline(
        &self,
        boundary: CurveBoundary<Point<1>>,
        _: Tolerance,
    ) -> Vec<Point<1>> {
        boundary.inner.into()
    }
}
