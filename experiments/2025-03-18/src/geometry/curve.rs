use fj_interop::{CircleApproxParams, Tolerance};
use fj_math::{Point, Vector};

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
    pub fn from_origin_and_curve(
        origin: Point<3>,
        curve: impl CurveGeometry + 'static,
    ) -> Self {
        Self {
            origin,
            floating: FloatingCurve {
                inner: Box::new(curve),
            },
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
        self.origin + self.floating.inner.vector_from_local_point(point.into())
    }

    pub fn project_point(&self, point: Point<3>) -> Point<1> {
        self.floating.inner.project_vector(point - self.origin)
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            origin: self.origin + offset,
            floating: FloatingCurve {
                inner: self.floating.inner.clone_curve_geometry(),
            },
        }
    }

    pub fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> Vec<Point<1>> {
        self.floating.inner.approximate(boundary, tolerance)
    }
}

impl Clone for AnchoredCurve {
    fn clone(&self) -> Self {
        Self {
            origin: self.origin,
            floating: FloatingCurve {
                inner: self.floating.inner.clone_curve_geometry(),
            },
        }
    }
}

pub struct FloatingCurve {
    pub inner: Box<dyn CurveGeometry>,
}

pub trait CurveGeometry {
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
    ) -> Vec<Point<1>>;
}

impl CurveGeometry for Circle {
    fn clone_curve_geometry(&self) -> Box<dyn CurveGeometry> {
        Box::new(*self)
    }

    fn vector_from_local_point(&self, point: Point<1>) -> Vector<3> {
        self.vector_from_local_point(point)
    }

    fn project_vector(&self, vector: Vector<3>) -> Point<1> {
        self.project_vector(vector)
    }

    fn flip(&self) -> Box<dyn CurveGeometry> {
        Box::new(Circle {
            a: self.a,
            b: -self.b,
        })
    }

    fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> Vec<Point<1>> {
        CircleApproxParams::new(self.radius(), tolerance)
            .approx_circle(boundary)
            .collect()
    }
}

impl CurveGeometry for Line {
    fn clone_curve_geometry(&self) -> Box<dyn CurveGeometry> {
        Box::new(*self)
    }

    fn vector_from_local_point(&self, point: Point<1>) -> Vector<3> {
        self.vector_from_local_point(point)
    }

    fn project_vector(&self, vector: Vector<3>) -> Point<1> {
        self.project_vector(vector)
    }

    fn flip(&self) -> Box<dyn CurveGeometry> {
        Box::new(Line {
            direction: -self.direction,
        })
    }

    fn approximate(&self, _: [Point<1>; 2], _: Tolerance) -> Vec<Point<1>> {
        vec![]
    }
}
