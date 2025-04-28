use fj_interop::{CircleApproxParams, Tolerance};
use fj_math::{Point, Transform, Vector};

use super::{Circle, Line};

/// # Curve geometry that has a fixed position (is _anchored_) in space
///
/// The opposite would be _floating_ curve geometry, which could be relative to
/// any point.
///
/// In terms of a line, for example, the anchored version is the full line, an
/// origin and a direction (a point and a vector). The floating version is just
/// the direction (a vector).
pub struct AnchoredCurve {
    /// # The origin point of the curve, which anchors it in 3D space
    ///
    /// This _must always_ be the origin point of the curve's coordinate system.
    /// Using something like the center of a circle is not valid!
    pub origin: Point<3>,

    /// # The floating part of the curve geometry
    pub floating: FloatingCurve,
}

impl AnchoredCurve {
    pub fn line_from_origin_and_direction(
        origin: Point<3>,
        direction: Vector<3>,
    ) -> Self {
        let line = Line { direction };

        Self {
            origin,
            floating: Box::new((origin, line)),
        }
    }

    pub fn line_from_points([a, b]: [Point<3>; 2]) -> Self {
        let origin = a;
        let direction = b - a;

        Self::line_from_origin_and_direction(origin, direction)
    }

    pub fn point_from_local(&self, point: Point<1>) -> Point<3> {
        self.floating.point_from_local(point)
    }

    pub fn project_point(&self, point: Point<3>) -> Point<1> {
        self.floating.project_point(point)
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();

        Self {
            origin: Transform::translation(offset)
                .transform_point(&self.origin),
            floating: self.floating.translate(offset),
        }
    }

    pub fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> Vec<Point<1>> {
        self.floating.approximate(boundary, tolerance)
    }
}

impl Clone for AnchoredCurve {
    fn clone(&self) -> Self {
        Self {
            origin: self.origin,
            floating: self.floating.clone_curve_geometry(),
        }
    }
}

pub type FloatingCurve = Box<dyn CurveGeometry>;

pub trait CurveGeometry {
    fn clone_curve_geometry(&self) -> FloatingCurve;
    fn point_from_local(&self, point: Point<1>) -> Point<3>;
    fn project_point(&self, point: Point<3>) -> Point<1>;
    fn translate(&self, offset: Vector<3>) -> FloatingCurve;

    /// # Approximate the curve
    ///
    /// Returns a list of points, in curve coordinates, that approximate the
    /// curve. The points must be within the provided boundary. Not outside of
    /// it, and not on it.
    fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> Vec<Point<1>>;
}

impl CurveGeometry for (Point<3>, fj_math::Circle<3>) {
    fn clone_curve_geometry(&self) -> FloatingCurve {
        Box::new(*self)
    }

    fn point_from_local(&self, point: Point<1>) -> Point<3> {
        let (offset, circle) = *self;

        let circle = Circle {
            a: circle.a(),
            b: circle.b(),
        };

        offset + circle.vector_from_local_point(point)
    }

    fn project_point(&self, point: Point<3>) -> Point<1> {
        let (origin, circle) = *self;

        let circle = Circle {
            a: circle.a(),
            b: circle.b(),
        };

        circle.project_vector(point - origin)
    }

    fn translate(&self, offset: Vector<3>) -> FloatingCurve {
        let (origin, circle) = *self;

        let origin = origin + offset;
        let circle = circle.transform(&Transform::translation(offset));

        Box::new((origin, circle))
    }

    fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> Vec<Point<1>> {
        let (_, circle) = self;

        CircleApproxParams::new(circle, tolerance)
            .approx_circle(boundary)
            .collect()
    }
}

impl CurveGeometry for (Point<3>, Line) {
    fn clone_curve_geometry(&self) -> FloatingCurve {
        Box::new(*self)
    }

    fn point_from_local(&self, point: Point<1>) -> Point<3> {
        let (origin, line) = *self;

        origin + line.vector_from_local_point(point)
    }

    fn project_point(&self, point: Point<3>) -> Point<1> {
        let (origin, line) = *self;

        line.project_vector(point - origin)
    }

    fn translate(&self, offset: Vector<3>) -> FloatingCurve {
        let (origin, line) = *self;

        let origin = origin + offset;

        Box::new((origin, line))
    }

    fn approximate(&self, _: [Point<1>; 2], _: Tolerance) -> Vec<Point<1>> {
        vec![]
    }
}
