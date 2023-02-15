use num_traits::Float;

use crate::{Point, Scalar, Vector};

/// Calculated geometry that is useful when dealing with an arc
pub struct Arc {
    /// Center of the circle the arc is constructed on
    pub center: Point<2>,

    /// Radius of the circle the arc is constructed on
    pub radius: Scalar,

    /// Angle of `start` relative to `center`, in radians
    ///
    /// Guaranteed to be less than `end_angle`.
    pub start_angle: Scalar,

    /// Angle of `end` relative to `center`, in radians
    ///
    /// Guaranteed to be greater than `end_angle`.
    pub end_angle: Scalar,

    /// True if `start` and `end` were switched to ensure `end_angle` > `start_angle`
    pub flipped_construction: bool,
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

        // This is an implementation of this solution:
        // https://math.stackexchange.com/a/87374

        let distance_between_endpoints = (p1 - p0).magnitude();
        let radius = distance_between_endpoints
            / (2. * (angle_rad.abs().into_f64() / 2.).sin());
        let distance_center_to_midpoint =
            (radius.powi(2) - (distance_between_endpoints.powi(2) / 4.)).sqrt();

        let flipped_construction = angle_rad <= Scalar::ZERO;
        let angle_rad = angle_rad.abs();

        let [p0, p1] = if flipped_construction {
            [p1, p0]
        } else {
            [p0, p1]
        };

        let (uv_factor, end_angle_offset) = if angle_rad > Scalar::PI {
            (Scalar::from_f64(-1.), Scalar::TAU)
        } else {
            (Scalar::ONE, Scalar::ZERO)
        };
        let [[x0, y0], [x1, y1]] = [p0, p1].map(|p| p.coords.components);
        let unit_vector_p0_to_p1 =
            (p1 - p0) / distance_between_endpoints * uv_factor;
        let unit_vector_midpoint_to_center =
            Vector::from([-unit_vector_p0_to_p1.v, unit_vector_p0_to_p1.u]);
        // (cx, cy) is the center of the circle
        let center = Point {
            coords: (p0.coords + p1.coords) / 2.
                + unit_vector_midpoint_to_center * distance_center_to_midpoint,
        };
        let start_angle = (y0 - center.v).atan2(x0 - center.u);
        let end_angle = (y1 - center.v).atan2(x1 - center.u) + end_angle_offset;
        Self {
            center,
            radius,
            start_angle,
            end_angle,
            flipped_construction,
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
        dbg!(arc.flipped_construction);
        assert_abs_diff_eq!(arc.center, center, epsilon = epsilon);
        assert_abs_diff_eq!(
            arc.radius,
            Scalar::from(radius),
            epsilon = epsilon
        );

        if a0 < a1 {
            assert!(!arc.flipped_construction);
            assert_abs_diff_eq!(
                arc.start_angle,
                Scalar::from(a0),
                epsilon = epsilon
            );
            assert_abs_diff_eq!(
                arc.end_angle,
                Scalar::from(a1),
                epsilon = epsilon
            );
        } else {
            assert!(arc.flipped_construction);
            assert_abs_diff_eq!(
                arc.end_angle,
                Scalar::from(a0),
                epsilon = epsilon
            );
            assert_abs_diff_eq!(
                arc.start_angle,
                Scalar::from(a1),
                epsilon = epsilon
            );
        }
    }
}
