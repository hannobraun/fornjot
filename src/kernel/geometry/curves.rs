use std::f64::consts::PI;

use crate::math::Point;

/// A one-dimensional shape
///
/// The word "curve" is used as an umbrella term for all one-dimensional shapes,
/// and doesn't imply that those shapes need to be curved. Straight lines are
/// included.
///
/// The nomenclature is inspired by Boundary Representation Modelling Techniques
/// by Ian Stroud. "Curve" refers to unbounded one-dimensional geometry, while
/// while edges are bounded portions of curves.
///
/// This distinction is not observed here, but moving things into that direction
/// is the intention.
pub enum Curve {
    /// A circle
    Circle(Circle),

    /// A line
    Line(Line),
}

/// A circle
///
/// This representation is not optimal, for two reasons:
/// - It doesn't define the center point of the circle. For that reason,
///   only circles centered on the origin are supported at this point.
/// - It doesn't define where the circle begins. For the purposes of
///   defining an arc on the circle, the zero angle will implicitly be to
///   the right.
///
/// It might be better to define a circle using two points: The center, and
/// the "zero" point on the circumference.
pub struct Circle {
    /// The radius of the circle
    pub radius: f64,
}

impl Circle {
    pub fn approx_vertices(&self, tolerance: f64) -> Vec<Point> {
        let angle_to_point = |angle: f64| {
            let (sin, cos) = angle.sin_cos();

            let x = cos * self.radius;
            let y = sin * self.radius;

            [x, y, 0.].into()
        };

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
        }

        let mut vertices = Vec::new();

        let first_vertex = angle_to_point(0.0);
        vertices.push(first_vertex);

        for i in 1..n {
            let angle = 2. * PI / n as f64 * i as f64;
            vertices.push(angle_to_point(angle));
        }

        // Connect the circle's to itself.
        vertices.push(first_vertex);

        vertices
    }
}

/// A line, defined by two points
pub struct Line {
    /// One point defining the line
    pub a: Point,

    /// The other point defining the line
    pub b: Point,
}
