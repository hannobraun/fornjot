use std::f64::consts::PI;

use nalgebra::vector;
use parry3d_f64::math::Isometry;

use crate::math::{Point, Vector};

/// A circle
#[derive(Clone, Debug)]
pub struct Circle {
    /// The center point of the circle
    pub center: Point<3>,

    /// The radius of the circle
    ///
    /// The radius is represented by a vector that points from the center to the
    /// circumference. The point on the circumference that it points to defines
    /// the origin of the circle's 1-dimensional curve coordinate system.
    pub radius: Vector<3>,
}

impl Circle {
    pub fn transform(&mut self, transform: &Isometry<f64>) {
        self.center = transform.transform_point(&self.center);
        self.radius = transform.transform_vector(&self.radius);
    }

    pub fn approx_vertices(&self, tolerance: f64, out: &mut Vec<Point<3>>) {
        let radius = self.radius.magnitude();

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
            let incircle_radius = radius * (PI / n as f64).cos();
            let maximum_error = radius - incircle_radius;

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

            let x = cos * radius;
            let y = sin * radius;

            let point = self.center + vector![x, y, 0.];

            out.push(point);
        }
    }
}
