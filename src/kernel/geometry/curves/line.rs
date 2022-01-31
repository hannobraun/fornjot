use approx::AbsDiffEq;
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
    pub fn transform(&mut self, transform: &Isometry<f64>) {
        self.a = transform.transform_point(&self.a);
        self.b = transform.transform_point(&self.b);
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
        let mut line = Line {
            a: point![1., 0., 0.],
            b: point![1., 1., 0.],
        };

        line.transform(&Isometry::from_parts(
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
}
