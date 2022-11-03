use fj_math::{Point, Scalar, Vector};

use crate::{partial::PartialCurve, path::SurfacePath};

/// Builder API for [`PartialCurve`]
#[allow(clippy::wrong_self_convention)]
pub trait CurveBuilder {
    /// Update partial curve to represent the u-axis
    fn update_as_u_axis(self) -> Self;

    /// Update partial curve to represent the v-axis
    fn as_v_axis(self) -> Self;

    /// Update partial curve as a circle, from the provided radius
    fn as_circle_from_radius(self, radius: impl Into<Scalar>) -> Self;

    /// Update partial curve as a line, from the provided points
    fn as_line_from_points(self, points: [impl Into<Point<2>>; 2]) -> Self;
}

impl CurveBuilder for PartialCurve {
    fn update_as_u_axis(self) -> Self {
        let a = Point::origin();
        let b = a + Vector::unit_u();

        self.as_line_from_points([a, b])
    }

    fn as_v_axis(self) -> Self {
        let a = Point::origin();
        let b = a + Vector::unit_v();

        self.as_line_from_points([a, b])
    }

    fn as_circle_from_radius(self, radius: impl Into<Scalar>) -> Self {
        self.with_path(Some(SurfacePath::circle_from_radius(radius)))
    }

    fn as_line_from_points(self, points: [impl Into<Point<2>>; 2]) -> Self {
        self.with_path(Some(SurfacePath::line_from_points(points)))
    }
}
