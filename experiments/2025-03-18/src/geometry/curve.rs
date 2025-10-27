use std::fmt;

use fj_interop::Tolerance;
use fj_math::{Point, Scalar, Vector};

use crate::approx::curve::{
    CurveApprox, PartialAnchoredCurveApprox, PartialCurveFloatingApprox,
};

use super::Line;

/// # Curve geometry that has a fixed position (is _anchored_) in space
///
/// The opposite would be _floating_ curve geometry, which could be relative to
/// any point.
///
/// In terms of a line, for example, the anchored version is the full line, an
/// origin and a direction (a point and a vector). The floating version is just
/// the direction (a vector).
#[derive(Clone, Debug)]
pub struct CurveAnchored {
    /// # The origin point of the curve, which anchors it in 3D space
    ///
    /// This _must always_ be the origin point of the curve's coordinate system.
    /// Using something like the center of a circle is not valid!
    pub origin: Point<3>,

    /// # The floating part of the curve geometry
    pub floating: CurveFloating,
}

impl CurveAnchored {
    pub fn from_origin_and_curve(
        origin: Point<3>,
        curve: impl CurveGeometry + 'static,
    ) -> Self {
        Self {
            origin,
            floating: CurveFloating::new(curve),
        }
    }

    pub fn line_from_origin_and_direction(
        origin: Point<3>,
        direction: Vector<3>,
    ) -> Self {
        let line = Line { direction };
        Self::from_origin_and_curve(origin, line)
    }

    pub fn line_from_points([a, b]: [Point<3>; 2]) -> Self {
        let origin = a;
        let direction = b - a;

        Self::line_from_origin_and_direction(origin, direction)
    }

    pub fn point_from_local(&self, point: impl Into<Point<1>>) -> Point<3> {
        self.origin + self.floating.vector_from_local_point(point.into())
    }

    pub fn project_point(&self, point: Point<3>) -> Point<1> {
        self.floating.geometry.project_vector(point - self.origin)
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            origin: self.origin + offset,
            floating: self.floating.clone(),
        }
    }

    /// # Approximate the curve
    ///
    /// Calls [`CurveGeometry::approximate`] internally and follows the same
    /// contract.
    pub fn approximate(
        &self,
        boundary: [impl Into<Point<1>>; 2],
        tolerance: Tolerance,
    ) -> PartialAnchoredCurveApprox {
        self.floating
            .approximate(boundary, tolerance)
            .into_anchored(self.origin, &*self.floating.geometry)
    }
}

#[derive(Debug)]
pub struct CurveFloating {
    pub geometry: Box<dyn CurveGeometry>,
}

impl CurveFloating {
    pub fn new(curve: impl CurveGeometry + 'static) -> Self {
        Self {
            geometry: Box::new(curve),
        }
    }

    pub fn vector_from_local_point(
        &self,
        point: impl Into<Point<1>>,
    ) -> Vector<3> {
        self.geometry.vector_from_local_point(point.into())
    }

    pub fn flip(&self) -> Self {
        Self {
            geometry: self.geometry.flip(),
        }
    }

    /// # Approximate the curve
    ///
    /// Calls [`CurveGeometry::approximate`] internally and follows the same
    /// contract.
    pub fn approximate(
        &self,
        boundary: [impl Into<Point<1>>; 2],
        tolerance: Tolerance,
    ) -> PartialCurveFloatingApprox {
        let boundary = boundary.map(Into::into);

        let [a, b] = boundary;
        let direction = (b.t - a.t).sign();

        let [min, max] = if direction.is_positive() {
            [a, b]
        } else {
            [b, a]
        };

        let size_hint = max.t - min.t;

        let mut approx =
            CurveApprox::new(self.geometry.as_ref(), tolerance, size_hint);
        approx.expand_to_include(min);
        approx.expand_to_include(max);

        let mut curvature = approx.into_points();

        if direction.is_negative() {
            curvature.reverse();
        }

        PartialCurveFloatingApprox { curvature }
    }
}

impl Clone for CurveFloating {
    fn clone(&self) -> Self {
        Self {
            geometry: self.geometry.clone_curve_geometry(),
        }
    }
}

pub trait CurveGeometry: fmt::Debug {
    fn clone_curve_geometry(&self) -> Box<dyn CurveGeometry>;
    fn vector_from_local_point(&self, point: Point<1>) -> Vector<3>;
    fn project_vector(&self, vector: Vector<3>) -> Point<1>;
    fn flip(&self) -> Box<dyn CurveGeometry>;

    /// # Compute the increment for approximating the curve, at the given point
    ///
    /// Curves are approximated as polylines. To build a polyline from a curve,
    /// you need to know at which points on the curve a segment of the polyline
    /// begins/ends.
    ///
    /// This is defined by the increment. The increment defines a distance on
    /// the curve, which is the length of a segment of the polyline.
    ///
    /// The increment can vary along the curve, as parts of it can be curved
    /// more tightly than others, requiring more line segments to approximate
    /// while keeping the same precisions. For other curves, like circles or
    /// lines, the increment is constant throughout.
    ///
    /// Aside from the point at which to compute the increment, this method
    /// takes two more parameters: The tolerance, which is the maximum deviation
    /// between the curve and its approximation, and a size hint.
    ///
    /// In many cases, the tolerance is enough to compute the increment. Where a
    /// curve is flat though, the approximation will be perfectly accurate, and
    /// the tolerance has no meaning. For this case, a size hint is provided,
    /// which an implementation can use as the size of the increment.
    fn increment_at(
        &self,
        point: Point<1>,
        tolerance: Tolerance,
        size_hint: Scalar,
    ) -> Increment;
}

/// # The increment of a curve approximation, in curve space
///
/// See [`CurveGeometry::increment`].
#[derive(Clone, Copy)]
pub struct Increment {
    pub inner: Vector<1>,
}

impl Increment {
    pub fn snap_to_multiple(&self, point: Point<1>) -> Point<1> {
        (point / self.inner).floor() * self.inner
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Vector};

    use crate::geometry::{Circle, Line, curve::CurveGeometry};

    #[test]
    fn flip() {
        let circle = Circle {
            a: Vector::from([1., 0., 0.]),
            b: Vector::from([0., 1., 0.]),
        };
        let line = Line {
            direction: Vector::from([1., 0., 0.]),
        };

        check(circle);
        check(line);

        fn check(curve: impl CurveGeometry) {
            for i in 0..8 {
                let point = Point::from([i as f64]);

                assert_eq!(
                    curve.vector_from_local_point(point),
                    curve.flip().vector_from_local_point(-point)
                );
                assert_eq!(
                    curve.vector_from_local_point(-point),
                    curve.flip().vector_from_local_point(point)
                );
            }
        }
    }
}
