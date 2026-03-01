use std::iter;

use crate::{
    approx::Tolerance,
    math::{Point, Scalar, Sign},
};

/// # Approximation parameters for a circle
#[derive(Debug)]
pub struct CircleApproxParams {
    increment: Scalar,
}

impl CircleApproxParams {
    /// # Compute the approximation parameters for a given circle and tolerance
    pub fn new(
        radius: impl Into<Scalar>,
        tolerance: impl Into<Tolerance>,
    ) -> Self {
        let num_vertices_to_approx_full_circle = Scalar::max(
            Scalar::PI
                / (Scalar::ONE - (tolerance.into().inner() / radius)).acos(),
            3.,
        )
        .ceil();

        let increment = Scalar::TAU / num_vertices_to_approx_full_circle;

        Self { increment }
    }

    /// # Access the increment at which the circle will be approximated
    pub fn increment(&self) -> Scalar {
        self.increment
    }

    /// # Generate points to approximate the circle within a given boundary
    pub fn approx_circle(
        &self,
        boundary: [Point<1>; 2],
    ) -> impl Iterator<Item = Point<1>> + '_ {
        // The boundary, in units of the increment.
        let [a, b] = boundary.map(|point| point.t / self.increment);

        let direction = (b - a).sign();
        let [min, max] = if a < b { [a, b] } else { [b, a] };

        // We can't generate a point exactly at the boundaries of the range as
        // part of the approximation. Make sure we stay inside the range.
        //
        // `min` and `max` are in units of the increment, so adding or
        // subtracting `1` adds or subtracts one increment.
        let min = min.floor() + 1.;
        let max = max.ceil() - 1.;

        let [start, end] = match direction {
            Sign::Negative => [max, min],
            Sign::Positive | Sign::Zero => [min, max],
        };

        let mut i = start;
        iter::from_fn(move || {
            let is_finished = match direction {
                Sign::Negative => i < end,
                Sign::Positive | Sign::Zero => i > end,
            };

            if is_finished {
                return None;
            }

            let t = self.increment * i;
            i += direction.to_scalar();

            Some(Point::from([t]))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::TAU;

    use crate::{
        approx::Tolerance,
        math::{Circle, Point, Scalar},
    };

    use super::CircleApproxParams;

    #[test]
    fn increment_for_circle() {
        test_increment(1., 0.5, 3.);
        test_increment(1., 0.1, 7.);
        test_increment(1., 0.01, 23.);

        fn test_increment(
            radius: impl Into<Scalar>,
            tolerance: impl Into<Tolerance>,
            expected_num_vertices: impl Into<Scalar>,
        ) {
            let circle = Circle::from_center_and_radius([0., 0.], radius);
            let params = CircleApproxParams::new(circle.radius(), tolerance);

            let expected_increment = Scalar::TAU / expected_num_vertices;
            assert_eq!(params.increment(), expected_increment);
        }
    }

    #[test]
    fn points_for_circle() {
        // At the chosen values for radius and tolerance (see below), the
        // increment is `PI / 4`, so ~1.57.

        // Empty range
        let empty: [Scalar; 0] = [];
        test_path([0., 0.], empty);

        // Ranges contain all generated points. Start is before the first
        // increment and after the last one in each case.
        test_path([0., TAU], [1., 2., 3.]);
        test_path([1., TAU], [1., 2., 3.]);
        test_path([0., TAU - 1.], [1., 2., 3.]);

        // Here the range is restricted to cut of the first or last increment.
        test_path([2., TAU], [2., 3.]);
        test_path([0., TAU - 2.], [1., 2.]);

        // And everything again, but in reverse.
        test_path([TAU, 0.], [3., 2., 1.]);
        test_path([TAU, 1.], [3., 2., 1.]);
        test_path([TAU - 1., 0.], [3., 2., 1.]);
        test_path([TAU, 2.], [3., 2.]);
        test_path([TAU - 2., 0.], [2., 1.]);

        fn test_path(
            boundary: [f64; 2],
            expected_coords: impl IntoIterator<Item = impl Into<Scalar>>,
        ) {
            // Choose radius and tolerance such, that we need 4 vertices to
            // approximate a full circle. This is the lowest number that we can
            // still cover all the edge cases with
            let radius = 1.;
            let tolerance = 0.375;

            let circle = Circle::from_center_and_radius([0., 0.], radius);
            let params = CircleApproxParams::new(circle.radius(), tolerance);

            let points = params
                .approx_circle(boundary.map(|coord| Point::from([coord])))
                .collect::<Vec<_>>();

            let expected_points = expected_coords
                .into_iter()
                .map(|i| Point::from([params.increment() * i]))
                .collect::<Vec<_>>();
            assert_eq!(points, expected_points);
        }
    }
}
