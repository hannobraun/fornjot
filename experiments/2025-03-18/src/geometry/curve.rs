use std::fmt;

use fj_interop::Tolerance;
use fj_math::{Point, Scalar, Vector};

use crate::approx::curve::{CurveApproxAnchored, CurveApproxFloating};

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
    ) -> CurveApproxAnchored {
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
    ) -> CurveApproxFloating {
        let boundary = boundary.map(Into::into);
        self.geometry.approximate(boundary, tolerance)
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

    /// # Approximate the curve
    ///
    /// Returns a list of points, in curve coordinates, that approximate the
    /// curve. The points must be within the provided boundary. Not outside of
    /// it, and not on it.
    fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> CurveApproxFloating;

    fn increment(&self, tolerance: Tolerance, size_hint: Scalar) -> Vector<1>;
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
