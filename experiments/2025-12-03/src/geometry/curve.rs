use fj_math::{Circle, Point, Scalar, Vector};

// TASK: The use of `end` here is somewhat confused. It seems to be an absolute
//       point, and yet we have no start point here. All curves should be
//       relative, and `end` should be a vector.
// TASK: Make generic over dimensionality.
pub trait Curve {
    fn end(&self) -> Vector<2>;
    // TASK: Once curves are relative, rename this to something that makes more
    //       sense.
    fn end(&self) -> Point<2>;
    // TASK: If curves were truly relative, we could return a `Vec<Vector<2>`
    //       here and not require `start`.
    fn approx(&self, start: Point<2>) -> Vec<Point<2>>;
}

#[derive(Clone, Copy, Debug)]
pub struct Arc {
    pub end: Vector<2>,
    pub radius: Scalar,
    pub tolerance: Scalar,
}

impl Curve for Arc {
    fn end(&self) -> Vector<2> {
        self.end
    }

    fn approx(&self, start: Point<2>) -> Vec<Point<2>> {
        // TASK: Inline?
        let start_to_end = self.end;
        let midpoint = start + start_to_end * 0.5;

        let midpoint_towards_center =
            start_to_end.to_perpendicular().normalize()
                * self.radius.sign().to_scalar();

        let distance_from_midpoint_to_center = {
            // We're computing the required distance from a right
            // triangle:
            //
            // - `a` (leg): `midpoint` to `end`
            // - `b` (leg): `midpoint` to circle center (the distance
            //   we're looking for)
            // - `c` (hypotenuse): `end` to circle center (which is
            //   `radius`)

            let a = start_to_end.magnitude() / 2.;
            let c = self.radius;

            let b_squared = c * c - a * a;

            if b_squared < Scalar::ZERO {
                panic!(
                    "Radius of arc (`{radius}`) is too small: Must be \
                    at least half the distance between start \
                    (`{start:?}`) and end (`{to:?}`) points, or the \
                    arc is not possible.",
                    radius = self.radius,
                    to = self.end,
                );
            }

            b_squared.sqrt()
        };

        let center = midpoint
            + midpoint_towards_center * distance_from_midpoint_to_center;

        // This only works if `surface` is a plane, which checks out for
        // now.
        let circle = {
            let a = start;
            let b = center + (a - center).to_perpendicular();

            Circle::new(center, a - center, b - center)
        };

        let num_vertices_to_approx_full_circle = Scalar::max(
            Scalar::PI / (Scalar::ONE - (self.tolerance / self.radius)).acos(),
            3.,
        )
        .ceil();

        let increment =
            Vector::from([Scalar::TAU / num_vertices_to_approx_full_circle]);

        let end = circle.point_to_circle_coords(start + self.end);
        let start = circle.point_to_circle_coords(start);

        let mut approx = Vec::new();

        let mut point = start + increment;
        while point < end {
            approx.push(circle.point_from_circle_coords(point));
            point += increment;
        }

        approx
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub end: Vector<2>,
}

impl Curve for Line {
    fn end(&self) -> Vector<2> {
        self.end
    }

    fn approx(&self, _: Point<2>) -> Vec<Point<2>> {
        Vec::new()
    }
}
