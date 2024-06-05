use num_traits::Float;

use crate::{Point, Scalar, Vector};

/// Calculated geometry that is useful when dealing with an arc
#[derive(Debug)]
pub struct Arc {
    /// Center of the circle the arc is constructed on
    pub center: Point<2>,

    /// Radius of the circle the arc is constructed on
    pub radius: Scalar,

    /// Angle of `start` relative to `center`, in radians
    pub start_angle: Scalar,

    /// Angle of `end` relative to `center`, in radians
    pub end_angle: Scalar,
}

impl Arc {
    /// Constructs an [`Arc`] from two endpoints and the associated angle.
    pub fn from_endpoints_and_angle(
        p0: impl Into<Point<2>>,
        p1: impl Into<Point<2>>,
        angle_rad: Scalar,
    ) -> Self {
        let p0 = p0.into();
        let p1 = p1.into();

        // This is an adaptation of this:
        // https://math.stackexchange.com/a/87374

        let distance_between_endpoints = (p1 - p0).magnitude();
        let more_than_half_turn = angle_rad.abs() > Scalar::PI;

        let radius = distance_between_endpoints
            / (2. * (angle_rad.abs().into_f64() / 2.).sin());

        let center = {
            let midpoint = Point {
                coords: (p0.coords + p1.coords) / 2.,
            };
            let unit_vector_midpoint_to_center = {
                let clockwise_turn = angle_rad <= Scalar::ZERO;
                let f = match (clockwise_turn, more_than_half_turn) {
                    (false, false) | (true, true) => Scalar::ONE,
                    (false, true) | (true, false) => -Scalar::ONE,
                };

                let unit_vector_p0_to_p1 =
                    (p1 - p0) / distance_between_endpoints * f;

                Vector::from([-unit_vector_p0_to_p1.v, unit_vector_p0_to_p1.u])
            };
            let distance_center_to_midpoint = (radius.powi(2)
                - (distance_between_endpoints.powi(2) / 4.))
                .sqrt();

            midpoint
                + unit_vector_midpoint_to_center * distance_center_to_midpoint
        };

        let start_angle = {
            let from_center = p0 - center;
            from_center.v.atan2(from_center.u)
        };
        let end_angle = {
            let from_center = p1 - center;
            let offset = if more_than_half_turn {
                Scalar::TAU
            } else {
                Scalar::ZERO
            };

            from_center.v.atan2(from_center.u) + offset
        };
        Self {
            center,
            radius,
            start_angle,
            end_angle,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, Scalar, Vector};

    use super::Arc;

    use approx::{assert_abs_diff_eq, AbsDiffEq};

    #[test]
    fn arc_construction() {
        check_arc_calculation(
            [0., 0.],
            1.,
            0_f64.to_radians(),
            90_f64.to_radians(),
        );
        check_arc_calculation(
            [-4., 2.],
            1.5,
            5_f64.to_radians(),
            -5_f64.to_radians(),
        );
        check_arc_calculation(
            [3., 8.],
            3.,
            0_f64.to_radians(),
            100_f64.to_radians(),
        );
        check_arc_calculation(
            [1., -1.],
            1.,
            90_f64.to_radians(),
            180_f64.to_radians(),
        );
        check_arc_calculation(
            [0., 0.],
            1.,
            0_f64.to_radians(),
            270_f64.to_radians(),
        );
    }

    fn check_arc_calculation(
        center: impl Into<Point<2>>,
        radius: f64,
        a0: f64,
        a1: f64,
    ) {
        let center = center.into();
        let angle = a1 - a0;

        let p0 = center + Vector::from([a0.cos(), a0.sin()]) * radius;
        let p1 = center + Vector::from([a1.cos(), a1.sin()]) * radius;

        let arc = Arc::from_endpoints_and_angle(p0, p1, Scalar::from(angle));

        let epsilon = Scalar::default_epsilon() * 10.;

        dbg!(arc.start_angle);
        dbg!(arc.end_angle);
        assert_abs_diff_eq!(arc.center, center, epsilon = epsilon);
        assert_abs_diff_eq!(
            arc.radius,
            Scalar::from(radius),
            epsilon = epsilon
        );

        assert_abs_diff_eq!(
            arc.start_angle,
            Scalar::from(a0),
            epsilon = epsilon
        );
        assert_abs_diff_eq!(arc.end_angle, Scalar::from(a1), epsilon = epsilon);
    }
}
