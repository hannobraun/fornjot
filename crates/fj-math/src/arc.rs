use crate::{Point, Scalar};

/// Calculated geometry that is useful when dealing with an arc
pub struct Arc {
    /// Start point of the arc
    pub start: Point<2>,
    /// End point of the arc
    pub end: Point<2>,
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
        angle: Scalar,
    ) -> Self {
        use num_traits::Float;

        let (p0, p1) = (p0.into(), p1.into());

        let flipped_construction = angle <= Scalar::ZERO;
        let angle_rad = angle.abs();

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
        // https://math.stackexchange.com/questions/27535/how-to-find-center-of-an-arc-given-start-point-end-point-radius-and-arc-direc
        // distance between endpoints
        let d = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
        // radius
        let r = d / (2. * (angle_rad.into_f64() / 2.).sin());
        // distance from center to midpoint between endpoints
        let h = (r.powi(2) - (d.powi(2) / 4.)).sqrt();
        // (u, v) is the unit normal in the direction of p1 - p0
        let u = (x1 - x0) / d * uv_factor;
        let v = (y1 - y0) / d * uv_factor;
        // (cx, cy) is the center of the circle
        let cx = ((x0 + x1) / 2.) - h * v;
        let cy = ((y0 + y1) / 2.) + h * u;
        let start_angle = (y0 - cy).atan2(x0 - cx);
        let end_angle = (y1 - cy).atan2(x1 - cx) + end_angle_offset;
        Self {
            start: p0,
            end: p1,
            center: Point::from([cx, cy]),
            radius: r,
            start_angle,
            end_angle,
            flipped_construction,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, Scalar};

    use super::Arc;

    use approx::AbsDiffEq;

    fn check_arc_calculation(center: [f64; 2], radius: f64, a0: f64, a1: f64) {
        let angle = a1 - a0;

        let p0 = [center[0] + radius * a0.cos(), center[1] + radius * a0.sin()];
        let p1 = [center[0] + radius * a1.cos(), center[1] + radius * a1.sin()];

        let arc = Arc::from_endpoints_and_angle(p0, p1, Scalar::from(angle));

        let epsilon = Scalar::default_epsilon() * 10.;

        dbg!(center, arc.center);
        dbg!(arc.start_angle);
        dbg!(arc.end_angle);
        dbg!(arc.flipped_construction);
        assert!(arc.center.abs_diff_eq(&Point::from(center), epsilon));
        assert!(arc.radius.abs_diff_eq(&Scalar::from(radius), epsilon));

        if a0 < a1 {
            assert!(!arc.flipped_construction);
            assert!(arc.start_angle.abs_diff_eq(&Scalar::from(a0), epsilon));
            assert!(arc.end_angle.abs_diff_eq(&Scalar::from(a1), epsilon));
        } else {
            assert!(arc.flipped_construction);
            assert!(arc.end_angle.abs_diff_eq(&Scalar::from(a0), epsilon));
            assert!(arc.start_angle.abs_diff_eq(&Scalar::from(a1), epsilon));
        }
    }

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
}
