use crate::math::{Circle, Point, Scalar, Vector};

/// # A relative curve
///
/// Curves in Fornjot are always _relative_, meaning the user of this trait must
/// provide a start point to locate them in space.
pub trait Curve {
    /// # Access the (relative) end point of the curve
    ///
    /// Provides a vector that points from a user-provided start point to the
    /// end point of the curve.
    fn end(&self) -> Vector<3>;

    /// # Approximate the curve
    ///
    /// Returns a list of vectors (relative to a user-provided start point) that
    /// approximate the curve. If a tolerance value is required, to define how
    /// far the approximation is allowed to deviate from the idealized curve,
    /// then such a value must be encoded into the curve itself.
    fn approx(&self) -> Vec<Vector<3>>;
}

/// # An arc, relative to a user-provided start point
pub struct Arc {
    /// # The end of the arc, relative to the user-provided start point
    pub end: Vector<3>,

    /// # The direction of the arc at the start point
    ///
    /// This vector defines a tangent of the circle that the arc is part of.
    pub dir: Vector<3>,

    /// # The tolerance value of the arc
    ///
    /// Defines how far the arc's approximation is allowed to deviate from the
    /// idealized arc.
    pub tolerance: Scalar,
}

impl Arc {
    /// # Construct a relative arc to the given end point
    ///
    /// This is a convenience constructor that accepts any arguments that
    /// convert into the types of an `Arc`'s fields.
    pub fn to(
        end: impl Into<Vector<3>>,
        dir: impl Into<Vector<3>>,
        tolerance: impl Into<Scalar>,
    ) -> Self {
        Self {
            end: end.into(),
            dir: dir.into(),
            tolerance: tolerance.into(),
        }
    }
}

impl Curve for Arc {
    fn end(&self) -> Vector<3> {
        self.end
    }

    fn approx(&self) -> Vec<Vector<3>> {
        // To approximate the arc, we need the center point and the radius of
        // the circle that is is defined on. We would have both, if we had a
        // `center` vector pointing from the start point of the arc to the
        // center point of the circle.
        //
        // We know that this `center` vector must be coplanar with `self.end`
        // and `self.dir`, and perpendicular to `self.dir`. We can start by
        // computing a vector that fulfills these requirements.
        let dir_perp = self.end - self.end.vector_projecting_onto(&self.dir);

        // `dir_perp` is colinear with the `center` vector we seek:
        //
        // ```
        // center = t * dir_perp (1)
        // ```
        //
        // This gives us an equation with two unknowns, `center` and `t`, which
        // means we need another equation to solve it.
        //
        // And since both start and end are points on the circle, the vectors
        // between them and the center must have the same length:
        //
        // ```
        // |center| = |center - end|
        // |center|² = |center - end|²
        // center * center = (center - end) * (center - end)
        // center * center = center * center - 2 * center * end + end * end
        // 0 = -2 * center * end + end * end
        // 2 * center * end = end * end
        // center * end = (end * end) / 2 (2)
        // ```
        //
        // By substituting `center` from (1) in (2), we get `t`:
        //
        // ```
        // t * dir_perp * end = (end * end) / 2
        // t = (end * end) / (2 * dir_perp * end)
        // ```
        let t = (self.end.dot(&self.end)) / (dir_perp.dot(&self.end) * 2.);

        // By putting that back into (1), we get `center`.
        let center = dir_perp * t;
        let radius = center.magnitude();

        let start = Point::origin();

        let circle = {
            let center = start + center;
            let a = start - center;
            let b = (self.end - self.end.vector_projecting_onto(&a))
                .normalize()
                * radius;

            Circle::new(center, a, b)
        };

        let num_vertices_to_approx_full_circle = Scalar::max(
            Scalar::PI / (Scalar::ONE - (self.tolerance / radius)).acos(),
            3.,
        )
        .ceil();

        let increment =
            Vector::from([Scalar::TAU / num_vertices_to_approx_full_circle]);

        let start_local = circle.point_to_circle_coords(start);
        let end_local = circle.point_to_circle_coords(start + self.end);

        let mut approx = Vec::new();

        let mut point = start_local + increment;
        while point < end_local {
            approx.push(circle.point_from_circle_coords(point) - start);
            point += increment;
        }

        approx
    }
}
