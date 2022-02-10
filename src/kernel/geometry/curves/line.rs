use approx::AbsDiffEq;
use nalgebra::point;
use parry3d_f64::math::Isometry;

use crate::math::Point;

/// A line, defined by two points
///
/// The points that define the line also define the line's 1-dimensional curve
/// coordinate system. `a` defines the origin (`0.0`), `b` defines coordinate
/// `1.0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    /// The first point that defines the line
    pub a: Point<3>,

    /// The second point that defines the line
    pub b: Point<3>,
}

impl Line {
    /// Transform the line
    #[must_use]
    pub fn transform(self, transform: &Isometry<f64>) -> Self {
        Self {
            a: transform.transform_point(&self.a),
            b: transform.transform_point(&self.b),
        }
    }

    /// Convert a point in model coordinates to curve coordinates
    ///
    /// Ignores the distance of the point to the line, meaning points on the
    /// line will be converted to the curve coordinates of their projection on
    /// the line.
    ///
    /// This is done to make this method robust against floating point accuracy
    /// issues. Callers are advised to be careful about the points they pass, as
    /// the point not being on the line, intended or not, will not result in an
    /// error.
    pub fn point_model_to_curve(&self, point: &Point<3>) -> Point<1> {
        let p = point - self.a;
        let d = self.b - self.a;

        // scalar projection
        let t = p.dot(&d.normalize());

        point![t]
    }
}

impl AbsDiffEq for Line {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.a.abs_diff_eq(&other.a, epsilon)
            && self.b.abs_diff_eq(&other.b, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use approx::assert_abs_diff_eq;
    use nalgebra::{point, UnitQuaternion};
    use parry3d_f64::math::{Isometry, Translation};

    use crate::math::Vector;

    use super::Line;

    #[test]
    fn test_transform() {
        let line = Line {
            a: point![1., 0., 0.],
            b: point![1., 1., 0.],
        };

        let line = line.transform(&Isometry::from_parts(
            Translation::from([1., 2., 3.]),
            UnitQuaternion::from_axis_angle(&Vector::z_axis(), FRAC_PI_2),
        ));

        assert_abs_diff_eq!(
            line,
            Line {
                a: point![1., 3., 3.],
                b: point![0., 3., 3.],
            },
            epsilon = 1e-8,
        );
    }

    #[test]
    fn test_point_model_to_curve() {
        let line = Line {
            a: point![1., 0., 0.],
            b: point![2., 0., 0.],
        };

        assert_eq!(line.point_model_to_curve(&point![0., 0., 0.]), point![-1.]);
        assert_eq!(line.point_model_to_curve(&point![1., 0., 0.]), point![0.]);
        assert_eq!(line.point_model_to_curve(&point![2., 0., 0.]), point![1.]);
        assert_eq!(line.point_model_to_curve(&point![3., 0., 0.]), point![2.]);
    }
}
