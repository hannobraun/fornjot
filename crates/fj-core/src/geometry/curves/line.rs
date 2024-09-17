//! # Geometry code specific to lines

use fj_math::{Line, Point};

use crate::geometry::{CurveBoundary, GenPolyline, Tolerance};

impl<const D: usize> GenPolyline<D> for Line<D> {
    fn origin(&self) -> Point<D> {
        self.origin()
    }

    fn line_segment_at(&self, point: Point<1>, _: Tolerance) -> [Point<D>; 2] {
        // Collapse line segment into a point, as per documentation.
        let point = self.origin() + self.direction() * point.t;

        [point, point]
    }

    fn generate_polyline(
        &self,
        boundary: CurveBoundary<Point<1>>,
        _: Tolerance,
    ) -> Vec<Point<1>> {
        boundary.inner.into()
    }
}
