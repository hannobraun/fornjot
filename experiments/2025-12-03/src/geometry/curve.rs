use fj_math::{Circle, Point, Scalar, Vector};

pub trait Curve<const D: usize> {
    fn approx(&self) -> Vec<Vector<D>>;
}

/// # An arc
///
/// Like all curves, arcs are relative to a user-managed origin point.
///
/// ## Design Note
///
/// This struct only represents 2D arcs. But according to the architecture that
/// I've started to lay out here, it should be generic over dimension.
/// Unfortunately, this isn't quite as straight-forward as making the vector
/// generic, as just a vector and a radius are not enough to define an arc in
/// 3D.
///
/// A different representation is required, but I'm not sure what the best
/// option is. As part of the process of evaluating those options, I'd like to
/// document them here, to help me figure out which one to choose.
///
/// During my first evaluation pass, I identified the following criteria:
///
/// 1. Needs to work in both 2D and 3D, which the current representation doesn't
///    meet.
/// 2. Should be able to represent all possible arcs. For example, an arc
///    between points with a radius allows for four interpretations. The circle
///    center can be left or right of the line between start and end points. For
///    each variant, the arc can then be left or right.
///    The current representation can only define two of those options, using
///    the sign of the radius.
/// 3. Should have few, ideally no, constraints to uphold within the
///    representation. This is not a blocker, as upholding these constraints can
///    be as easy as making the struct fields private and only writing them in a
///    constructor. But ideally, I can avoid this complexity.
/// 4. Should be straight-forward, maybe even intuitive. Ideally, it is easy to
///    imagine what effects a change to the representation has on the arc. Bonus
///    points, if the representation also makes sense graphically; something you
///    could directly manipulate in a CAD GUI.
///    This is also not a blocker, as constructors and accessor methods could
///    translate from and to more intuitive representations, but it would be
///    nicer to not require that complexity.
/// 5. Should be easy to work with from internal code. This is the other side of
///    the previous point, which makes it easy to work with from the user's
///    code. Though I guess this isn't a blocker either, since you can always
///    convert to a more convenient representation and then work with that.
///
/// ### 1. Vectors from the center point to the start and end points
///
/// Evaluation of criteria:
///
/// 1. Straight-forward to generalize by simply generalizing the vectors.
/// 2. Completely ambiguous. Would require two additional bits of information.
/// 3. Both vectors must be of equal length.
/// 4. Given that the start and end points are probably known in most cases,
///    having to define a center point while upholding the constraint seems
///    onerous. Having to pass an enum or two boolean flags adds to the
///    complication.
/// 5. Unclear, but needing special-case code for 4 cases seems like it wouldn't
///    be very elegant.
///
/// Doesn't seem like a good solution, due to the drawbacks in 2. and 4.,
/// possibly 5.
///
/// ### 2. Vectors from the start point to the mid and end points
///
/// The mid point here would be the mid point of the arc, not the mid point on
/// the straight line between start and end.
///
/// 1. Straight-forward to generalize by simply generalizing the vectors.
/// 2. Unambiguous. The vector to the mid point obviously defines the side where
///    the arc is located. Less obviously, the distance of the midpoint from the
///    direct line between start and end only allows for one interpretation, as
///    for the location of the circle center.
/// 3. The second vector must not be parallel with the first one, which is more
///    of an edge case than a real constraint. It must not actually point at the
///    mid point directly, only towards it, as the approach would work either
///    way.
/// 4. Easy to understand, but that the second vector must only point in the
///    direction of the mid point, not directly at it, makes it slightly less
///    intuitive.
/// 5. Unclear.
///
/// Seems like a good option. Very similar to option 4., which is a tiny bit
/// more intuitive though.
///
/// ### 3. Vector from start to end, second one to define plane, plus radius
///
/// A straight-forward expansion of the current representation.
///
/// 1. Generalizable, by generalizing the vectors, but messy. The second vector
///    is meaningless in 2D, thus would have to be ignored.
/// 2. Can be made unambiguous. The sign of the radius can specify one bit of
///    information, the direction of the second vector the other.
/// 3. The vectors must not point in the same or opposite directions, or they
///    won't define a plane. The radius must be at least half half the length of
///    the first vector.
/// 4. Seems quite messy, with one vector only relevant for 3D, and multiple
///    ways to interpret how the second vector and the radius define the arc.
/// 5. Unclear.
///
/// A very messy option with seemingly no redeeming qualities to recommend it
/// over the other unambiguous but more elegant options.
///
/// ### 4. Vector from start to end and tangent vector from the start point
///
/// 1. Straight-forward to generalize by simply generalizing the vectors.
/// 2. Unambiguous.
/// 3. The vectors must not point in the same or opposite directions. So barely
///    a constraint; more like an edge case.
/// 4. Easy to understand. Both vectors have a clear and direct relation to the
///    arc. But defining it is complicated by the fact that the tangent vector
///    must be relative to the start-to-end vector, and hence must be different
///    for arcs of the same radius.
/// 5. Unclear.
#[derive(Clone, Copy, Debug)]
pub struct Arc2 {
    pub start_to_end: Vector<2>,
    pub radius: Scalar,
    pub tolerance: Scalar,
}

impl Arc2 {
    pub fn from_vector_and_radius(
        start_to_end: Vector<2>,
        radius: Scalar,
        tolerance: Scalar,
    ) -> Self {
        Self {
            start_to_end,
            radius,
            tolerance,
        }
    }
}

impl Curve<2> for Arc2 {
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
