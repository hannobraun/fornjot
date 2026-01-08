use fj_math::{Circle, Point, Scalar, Vector};

pub trait Curve<const D: usize> {
    fn approx(&self) -> Vec<Vector<D>>;
}

#[derive(Clone, Copy, Debug)]
pub struct Arc {
    pub start_to_end: Vector<2>,
    pub radius: Scalar,
    pub tolerance: Scalar,
}

impl Curve<2> for Arc {
    fn approx(&self) -> Vec<Vector<2>> {
        let from = Point::origin();
        let midpoint = from + self.start_to_end * 0.5;

        let midpoint_towards_center = self.start_to_end.to_perpendicular().normalize()
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

            let a = self.start_to_end.magnitude() / 2.;
            let c = self.radius;

            let b_squared = c * c - a * a;

            if b_squared < Scalar::ZERO {
                panic!(
                    "Radius of arc (`{radius}`) is too small: Must be \
                    at least half the distance between start \
                    (`{from:?}`) and end (`{to:?}`) points, or the \
                    arc is not possible.",
                    radius = self.radius,
                    to = self.start_to_end,
                );
            }

            b_squared.sqrt()
        };

        let center = midpoint
            + midpoint_towards_center * distance_from_midpoint_to_center;

        // This only works if `surface` is a plane, which checks out for
        // now.
        let circle = {
            let a = from;
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

        let end = circle.point_to_circle_coords(from + self.start_to_end);
        let start = circle.point_to_circle_coords(from);

        let mut approx = Vec::new();

        let mut point = start + increment;
        while point < end {
            approx.push(circle.point_from_circle_coords(point) - from);
            point += increment;
        }

        approx
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line<const D: usize> {}

impl<const D: usize> Curve<D> for Line<D> {
    fn approx(&self) -> Vec<Vector<D>> {
        Vec::new()
    }
}
