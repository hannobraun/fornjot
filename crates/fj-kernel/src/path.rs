//! Paths through 2D and 3D space
//!
//! See [`SurfacePath`] and [`GlobalPath`].
//!
//! # Implementation Note
//!
//! This is a bit of an in-between module. It is closely associated with curves
//! ([`Curve`]/[`GlobalCurve`]) and [`Surface`]s, but paths are not really
//! objects themselves, as logically speaking, they are owned and not referenced
//! (practically speaking, all objects are owned and not referenced, but that is
//! an implementation detail; see [#1021] for context on where things are
//! going).
//!
//! On the other hand, the types in this module don't follow the general style
//! of types in `fj-math`.
//!
//! We'll see how it shakes out. Maybe it will stay its own thing, maybe it will
//! move to `fj-math`, maybe something else entirely will happen.
//!
//! [`Curve`]: crate::objects::Curve
//! [`GlobalCurve`]: crate::objects::GlobalCurve
//! [`Surface`]: crate::objects::Surface
//! [#1021]: https://github.com/hannobraun/Fornjot/issues/1021

use std::cmp::max;

use fj_math::{Circle, Line, Point, Scalar, Vector};

use crate::algorithms::approx::{curve::RangeOnPath, ApproxPoint, Tolerance};

/// A path through surface (2D) space
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SurfacePath {
    /// A circle
    Circle(Circle<2>),

    /// A line
    Line(Line<2>),
}

impl SurfacePath {
    /// Build a circle from the given radius
    pub fn circle_from_radius(radius: impl Into<Scalar>) -> Self {
        let radius = radius.into();

        SurfacePath::Circle(Circle::new(
            Point::origin(),
            Vector::from([radius, Scalar::ZERO]),
            Vector::from([Scalar::ZERO, radius]),
        ))
    }

    /// Construct a line from two points
    pub fn line_from_points(points: [impl Into<Point<2>>; 2]) -> Self {
        Self::Line(Line::from_points(points))
    }

    /// Convert a point on the path into global coordinates
    pub fn point_from_path_coords(
        &self,
        point: impl Into<Point<1>>,
    ) -> Point<2> {
        match self {
            Self::Circle(circle) => circle.point_from_circle_coords(point),
            Self::Line(line) => line.point_from_line_coords(point),
        }
    }
}

/// A path through global (3D) space
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GlobalPath {
    /// A circle
    Circle(Circle<3>),

    /// A line
    Line(Line<3>),
}

impl GlobalPath {
    /// Construct a `GlobalPath` that represents the x-axis
    pub fn x_axis() -> Self {
        Self::Line(Line::from_origin_and_direction(
            Point::origin(),
            Vector::unit_x(),
        ))
    }

    /// Construct a `GlobalPath` that represents the y-axis
    pub fn y_axis() -> Self {
        Self::Line(Line::from_origin_and_direction(
            Point::origin(),
            Vector::unit_y(),
        ))
    }

    /// Construct a `GlobalPath` that represents the z-axis
    pub fn z_axis() -> Self {
        Self::Line(Line::from_origin_and_direction(
            Point::origin(),
            Vector::unit_z(),
        ))
    }

    /// Build a circle from the given radius
    pub fn circle_from_radius(radius: impl Into<Scalar>) -> Self {
        let radius = radius.into();

        GlobalPath::Circle(Circle::new(
            Point::origin(),
            Vector::from([radius, Scalar::ZERO, Scalar::ZERO]),
            Vector::from([Scalar::ZERO, radius, Scalar::ZERO]),
        ))
    }

    /// Construct a line from two points
    pub fn line_from_points(points: [impl Into<Point<3>>; 2]) -> Self {
        Self::Line(Line::from_points(points))
    }

    /// Access the origin of the path's coordinate system
    pub fn origin(&self) -> Point<3> {
        match self {
            Self::Circle(circle) => circle.center() + circle.a(),
            Self::Line(line) => line.origin(),
        }
    }

    /// Convert a point on the path into global coordinates
    pub fn point_from_path_coords(
        &self,
        point: impl Into<Point<1>>,
    ) -> Point<3> {
        match self {
            Self::Circle(circle) => circle.point_from_circle_coords(point),
            Self::Line(line) => line.point_from_line_coords(point),
        }
    }

    /// Convert a vector on the path into global coordinates
    pub fn vector_from_path_coords(
        &self,
        vector: impl Into<Vector<1>>,
    ) -> Vector<3> {
        match self {
            Self::Circle(circle) => circle.vector_from_circle_coords(vector),
            Self::Line(line) => line.vector_from_line_coords(vector),
        }
    }

    /// Approximate the path
    pub fn approx(
        &self,
        range: RangeOnPath,
        tolerance: impl Into<Tolerance>,
    ) -> Vec<ApproxPoint<1>> {
        match self {
            GlobalPath::Circle(circle) => {
                approx_circle(circle, range, tolerance.into())
            }
            GlobalPath::Line(_) => vec![],
        }
    }
}

/// Approximate a circle
///
/// `tolerance` specifies how much the approximation is allowed to deviate
/// from the circle.
fn approx_circle(
    circle: &Circle<3>,
    range: impl Into<RangeOnPath>,
    tolerance: Tolerance,
) -> Vec<ApproxPoint<1>> {
    let mut points = Vec::new();

    let radius = circle.a().magnitude();
    let range = range.into();

    // To approximate the circle, we use a regular polygon for which
    // the circle is the circumscribed circle. The `tolerance`
    // parameter is the maximum allowed distance between the polygon
    // and the circle. This is the same as the difference between
    // the circumscribed circle and the incircle.

    let n = number_of_vertices_for_circle(tolerance, radius, range.length());

    for i in 1..n {
        let angle = range.start().t
            + (Scalar::TAU / n as f64 * i as f64) * range.direction();

        let point_curve = Point::from([angle]);
        let point_global = circle.point_from_circle_coords(point_curve);

        points.push(ApproxPoint::new(point_curve, point_global));
    }

    if range.is_reversed() {
        points.reverse();
    }

    points
}

fn number_of_vertices_for_circle(
    tolerance: Tolerance,
    radius: Scalar,
    range: Scalar,
) -> u64 {
    let n = (range / (Scalar::ONE - (tolerance.inner() / radius)).acos() / 2.)
        .ceil()
        .into_u64();

    max(n, 3)
}

#[cfg(test)]
mod tests {
    use fj_math::Scalar;

    use crate::algorithms::approx::Tolerance;

    #[test]
    fn number_of_vertices_for_circle() {
        verify_result(50., 100., Scalar::TAU, 3);
        verify_result(50., 100., Scalar::PI, 3);
        verify_result(10., 100., Scalar::TAU, 7);
        verify_result(10., 100., Scalar::PI, 4);
        verify_result(1., 100., Scalar::TAU, 23);
        verify_result(1., 100., Scalar::PI, 12);

        fn verify_result(
            tolerance: impl Into<Tolerance>,
            radius: impl Into<Scalar>,
            range: impl Into<Scalar>,
            n: u64,
        ) {
            let tolerance = tolerance.into();
            let radius = radius.into();
            let range = range.into();

            assert_eq!(
                n,
                super::number_of_vertices_for_circle(tolerance, radius, range)
            );

            assert!(calculate_error(radius, range, n) <= tolerance.inner());
            if n > 3 {
                assert!(
                    calculate_error(radius, range, n - 1) >= tolerance.inner()
                );
            }
        }

        fn calculate_error(radius: Scalar, range: Scalar, n: u64) -> Scalar {
            radius - radius * (range / Scalar::from_u64(n) / 2.).cos()
        }
    }
}
