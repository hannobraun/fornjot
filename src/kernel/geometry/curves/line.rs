use approx::AbsDiffEq;
use nalgebra::point;
use parry3d_f64::math::Isometry;

use crate::math::{Point, Vector};

/// A line, defined by a point and a vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    /// The origin of the line's coordinate system
    pub origin: Point<3>,

    /// The direction of the line
    ///
    /// The length of this vector defines the unit of the line's curve
    /// coordinate system. The coordinate `1.` is always were the direction
    /// vector points, from `origin`.
    pub direction: Vector<3>,
}

impl Line {
    /// Transform the line
    #[must_use]
    pub fn transform(self, transform: &Isometry<f64>) -> Self {
        Self {
            origin: transform.transform_point(&self.origin),
            direction: transform.transform_vector(&self.direction),
        }
    }

    /// Convert a point in model coordinates to curve coordinates
    ///
    /// Projects the point onto the line before computing curve coordinate. This
    /// is done to make this method robust against floating point accuracy
    /// issues.
    ///
    /// Callers are advised to be careful about the points they pass, as the
    /// point not being on the line, intentional or not, will never result in an
    /// error.
    pub fn point_model_to_curve(&self, point: &Point<3>) -> Point<1> {
        let p = point - self.origin;

        // scalar projection
        let t = p.dot(&self.direction.normalize()) / self.direction.magnitude();

        point![t]
    }
}

impl AbsDiffEq for Line {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.origin.abs_diff_eq(&other.origin, epsilon)
            && self.direction.abs_diff_eq(&other.direction, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use approx::assert_abs_diff_eq;
    use nalgebra::{point, vector, UnitQuaternion};
    use parry3d_f64::math::{Isometry, Translation};

    use crate::math::Vector;

    use super::Line;

    #[test]
    fn transform() {
        let line = Line {
            origin: point![1., 0., 0.],
            direction: vector![0., 1., 0.],
        };

        let line = line.transform(&Isometry::from_parts(
            Translation::from([1., 2., 3.]),
            UnitQuaternion::from_axis_angle(&Vector::z_axis(), FRAC_PI_2),
        ));

        assert_abs_diff_eq!(
            line,
            Line {
                origin: point![1., 3., 3.],
                direction: vector![-1., 0., 0.],
            },
            epsilon = 1e-8,
        );
    }

    #[test]
    fn point_model_to_curve() {
        let line = Line {
            origin: point![1., 0., 0.],
            direction: vector![2., 0., 0.],
        };

        verify(line, -1.);
        verify(line, 0.);
        verify(line, 1.);
        verify(line, 2.);

        fn verify(line: Line, t: f64) {
            let point = line.origin + line.direction * t;
            let t_result = line.point_model_to_curve(&point);

            assert_eq!(point![t], t_result);
        }
    }
}
