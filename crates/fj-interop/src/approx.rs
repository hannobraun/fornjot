use std::iter;

use fj_math::{Point, Scalar, Sign};

use crate::Tolerance;

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
        let [a, b] = boundary.map(|point| point.t / self.increment);
        let direction = (b - a).sign();
        let [min, max] = if a < b { [a, b] } else { [b, a] };

        // We can't generate a point exactly at the boundaries of the range as
        // part of the approximation. Make sure we stay inside the range.
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
