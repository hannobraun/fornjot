use fj_math::{Circle, Point, Scalar, Vector};

pub trait Curve<const D: usize> {
    fn approx(&self) -> Vec<Vector<D>>;
}

/// # An arc
///
/// ## Design Note
///
/// This struct only represents 2D arcs. But according to the architecture that
/// I've started to lay out here, it should be generic over dimension.
/// Unfortunately this isn't quite as straight-forward as making the vector
/// generic, as just a vector and a radius are not enough to define a 3D arc.
///
/// A different representation is required, but I'm not sure what the best
/// option is. I'd like to document the options I've considered here, to help me
/// figure out which one to choose.
///
/// ### Two vectors, from the center point to the start and end points
///
/// This is straight-forward to make generic (just make the two vectors
/// generic), but requires that both vectors have the same length (as both
/// define the radius).
///
/// This constraint is probably fine, but I'd like to consider more options, in
/// case one of them is more elegant.
///
/// ### Two vectors, from the start to the mid and end points
///
/// The mid point here would be the mid point of the arc, not the mid point on
/// the straight line between start and end.
///
/// Also straight-forward to make generic, and has the additional advantage of
/// doing away with the center point, which in most use cases probably isn't
/// immediately available and has to be computed. Meanwhile the start and end
/// points are readily available.
///
/// And the mid point might even provide a more intuitive definition of the arc
/// than the radius does.
///
/// However, the constraints that have to be upheld are even more convoluted, as
/// the second vector has to point to a point on a perpendicular line through
/// the middle of the second one.
///
/// ### Start to end vector, radius, and second vector to define the arc plane
///
/// This would be a straight-forward expansion of the current representation.
/// The only constraint would be that the second vector must not point in the
/// same direction as the first one. We could even encode the radius as the
/// length of the second vector, though that seems a bit convoluted.
///
/// And I guess that's my only complaint: That either we have one additional
/// scalar, or that the representation ends up convoluted and non-intuitive.
/// Neither seems like a blocker, but it would be nice to have something more
/// elegant.
///
/// ### Vector from start to end and tangent vector from the start point
///
/// Here we get a second vector that defines the tangent of the circle that
/// defines the arc, at the start point. This seems elegant, and maybe even
/// intuitive to the user (as in, I could see them manipulating that vector in a
/// graphical CAD interface).
///
/// The only constraint is that the second vector must not point into the same
/// or opposite direction from the first. Though at least them pointing in the
/// same direction could be special-cased as a straight line, which seems
/// somewhat intuitive.
///
/// Thinking about this made me realize that, unless I'm missing something, this
/// representation is also more powerful than the other ones. All of those seem
/// to assume, that the arc makes up the smaller part of the circle that defines
/// it, while the larger part is hidden.
///
/// While with this representation, both cases can easily be represented. If the
/// tangent vector points backwards (not completely opposite of the first
/// vector, but in that general direction), then it's obvious what the circle
/// and the arc should be.
#[derive(Clone, Copy, Debug)]
pub struct Arc {
    pub start_to_end: Vector<2>,
    pub radius: Scalar,
    pub tolerance: Scalar,
}

impl Curve<2> for Arc {
    fn approx(&self) -> Vec<Vector<2>> {
        let start = Point::origin();
        let radius = self.radius;

        let midpoint_towards_center =
            self.start_to_end.to_perpendicular().normalize()
                * radius.sign().to_scalar();

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
            let c = radius;

            let b_squared = c * c - a * a;

            if b_squared < Scalar::ZERO {
                panic!(
                    "Radius of arc (`{radius}`) is too small: Must be \
                    at least half the distance between start \
                    (`{start:?}`) and end (`{to:?}`) points, or the \
                    arc is not possible.",
                    to = self.start_to_end,
                );
            }

            b_squared.sqrt()
        };

        let center = start
            + self.start_to_end * 0.5
            + midpoint_towards_center * distance_from_midpoint_to_center;

        // This only works if `surface` is a plane, which checks out for
        // now.
        let circle = {
            let a = start;
            let b = center + (a - center).to_perpendicular();

            Circle::new(center, a - center, b - center)
        };

        let num_vertices_to_approx_full_circle = Scalar::max(
            Scalar::PI / (Scalar::ONE - (self.tolerance / radius)).acos(),
            3.,
        )
        .ceil();

        let increment =
            Vector::from([Scalar::TAU / num_vertices_to_approx_full_circle]);

        let start_local = circle.point_to_circle_coords(start);
        let end_local =
            circle.point_to_circle_coords(start + self.start_to_end);

        let mut approx = Vec::new();

        let mut point = start_local + increment;
        while point < end_local {
            approx.push(circle.point_from_circle_coords(point) - start);
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
