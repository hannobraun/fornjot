//! # Geometry code specific to circles

use crate::{
    approx::{CircleApprox, Tolerance},
    geometry::{CurveBoundary, traits::GenPolyline},
    math::{Circle, LineSegment, Point},
};

impl<const D: usize> GenPolyline<D> for Circle<D> {
    fn origin(&self) -> Point<D> {
        self.center() + self.a()
    }

    fn line_segment_at(
        &self,
        point_curve: Point<1>,
        tolerance: Tolerance,
    ) -> LineSegment<D> {
        let params = CircleApprox::new(self.radius(), tolerance);

        // The approximation parameters have an increment, in curve coordinates,
        // that determines the distance between points on the polyline. Let's
        // figure out where `point` is on the curve, in units of this increment.
        let t = point_curve.t / params.increment();

        // Now pick two points on the curve, again in units of approximation
        // increment, where the locations of the two closest approximation
        // points to the provided point are.
        //
        // Since we are calculating this in increment units, those are integer
        // numbers.
        let a = t.floor();
        let b = t.ceil();

        // Next, convert them into actual curve coordinates.
        let points_curve = [a, b]
            .map(|point_curve_in_increment_units| {
                [point_curve_in_increment_units * params.increment()]
            })
            .map(Point::from);

        // And finally, convert those into points of the desired dimensionality.
        let points = points_curve
            .map(|point_curve| self.point_from_circle_coords(point_curve));

        LineSegment {
            points,
            points_line: points_curve,
        }
    }

    fn generate_polyline(
        &self,
        boundary: CurveBoundary<Point<1>>,
        tolerance: Tolerance,
    ) -> Vec<Point<1>> {
        let params = CircleApprox::new(self.radius(), tolerance);
        params.approx_circle(boundary.inner).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        approx::Tolerance,
        geometry::traits::GenPolyline,
        math::{Circle, Point},
    };

    #[test]
    fn curve_representation_must_be_deterministic() -> anyhow::Result<()> {
        let circle = Circle::from_center_and_radius([0., 0.], 1.);

        // Deliberately choose a very coarse tolerance, so the circle
        // representation degenerates to a predictable triangle.
        let tolerance = Tolerance::from_scalar(1.)?;

        // Sample the circle at two points that are close together, relative to
        // our tolerance. The intent here is to each time sample the same
        // triangle edge, so also make sure they're not around zero, or another
        // point where two edges are likely to meet.
        //
        // Where those edges meet is implementation-dependent of course, so this
        // test might break if that implementation changes. But I don't think
        // that really matters. We just need to make sure that this test doesn't
        // accidentally hit such a point. Where specifically those points are,
        // doesn't matter.
        let a = circle.line_segment_at(Point::from([0.2]), tolerance);
        let b = circle.line_segment_at(Point::from([0.3]), tolerance);

        assert_eq!(
            a, b,
            "Expecting representation of the curve to be deterministic; it \
            must not depend on the specific points that were sampled.",
        );

        Ok(())
    }
}
