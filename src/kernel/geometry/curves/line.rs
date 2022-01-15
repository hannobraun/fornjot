use approx::AbsDiffEq;
use parry3d_f64::math::Isometry;

use crate::math::{Point, Vector};

/// A line, defined by two points
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    /// The origin point of the line
    ///
    /// The point on the plane that is the origin of the 1-dimensional curve
    /// coordinate system.
    pub origin: Point<3>,

    /// The direction of the line
    ///
    /// In addition to defining the direction of the line, the length of this
    /// vector defines the curve coordinate system: The point at `origin` +
    /// `dir` has curve coordinate `1.0`.
    pub dir: Vector<3>,
}

impl Line {
    /// Transform the line
    pub fn transform(&mut self, transform: &Isometry<f64>) {
        self.origin = transform.transform_point(&self.origin);
        self.dir = transform.transform_vector(&self.dir);
    }
}

impl AbsDiffEq for Line {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.origin.abs_diff_eq(&other.origin, epsilon)
            && self.dir.abs_diff_eq(&other.dir, epsilon)
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
    fn test_transform() {
        let mut line = Line {
            origin: point![1., 0., 0.],
            dir: vector![0., 1., 0.],
        };

        line.transform(&Isometry::from_parts(
            Translation::from([1., 2., 3.]),
            UnitQuaternion::from_axis_angle(&Vector::z_axis(), FRAC_PI_2),
        ));

        assert_abs_diff_eq!(
            line,
            Line {
                origin: point![1., 3., 3.],
                dir: vector![-1., 0., 0.]
            }
        );
    }
}
