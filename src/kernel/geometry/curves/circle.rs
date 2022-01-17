use std::f64::consts::PI;

use crate::math::Point;

/// A circle
///
/// This representation quite limited, for two reasons:
/// - It doesn't define the center point of the circle. For that reason,
///   only circles centered on the origin are supported at this point.
/// - It doesn't define where the circle begins. For the purposes of
///   defining an arc on the circle, the zero angle will implicitly be to
///   the right.
///
/// It might be better to define a circle using two points: The center, and
/// the "zero" point on the circumference.
#[derive(Clone, Debug)]
pub struct Circle {
    /// The center point of the circle
    pub center: Point<3>,

    /// The radius of the circle
    pub radius: f64,
}

impl Circle {
    pub fn approx_vertices(&self, tolerance: f64, out: &mut Vec<Point<3>>) {
        if self.center != Point::origin() {
            todo!(
                "Support for circles not centered at the origin is still \
                limited."
            )
        }

        // To approximate the circle, we use a regular polygon for which
        // the circle is the circumscribed circle. The `tolerance`
        // parameter is the maximum allowed distance between the polygon
        // and the circle. This is the same as the difference between
        // the circumscribed circle and the incircle.
        //
        // Let's figure which regular polygon we need to use, by just
        // trying out some of them until we find one whose maximum error
        // is less than or equal to the tolerance.
        let mut n = 3;
        loop {
            let incircle_radius = self.radius * (PI / n as f64).cos();
            let maximum_error = self.radius - incircle_radius;

            if maximum_error <= tolerance {
                break;
            }

            n += 1;

            // TASK: Log a warning, if `n` is becoming unreasonably large. This
            //       is either a bug, or the consequence of an unreasonably low
            //       value for `tolerance`.
        }

        for i in 0..n {
            let angle = 2. * PI / n as f64 * i as f64;

            let (sin, cos) = angle.sin_cos();

            let x = cos * self.radius;
            let y = sin * self.radius;

            let point = [x, y, 0.].into();

            out.push(point);
        }
    }
}
