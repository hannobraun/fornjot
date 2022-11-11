use fj_math::{Point, Scalar, Vector};

use crate::{partial::PartialCurve, path::SurfacePath};

/// Builder API for [`PartialCurve`]
pub trait CurveBuilder {
    /// Update partial curve to represent the u-axis
    fn update_as_u_axis(self) -> Self;

    /// Update partial curve to represent the v-axis
    fn update_as_v_axis(self) -> Self;

    /// Update partial curve as a circle, from the provided radius
    fn update_as_circle_from_radius(
        &mut self,
        radius: impl Into<Scalar>,
    ) -> &mut Self;

    /// Update partial curve as a line, from the provided points
    fn update_as_line_from_points(
        &mut self,
        points: [impl Into<Point<2>>; 2],
    ) -> &mut Self;
}

impl CurveBuilder for PartialCurve {
    fn update_as_u_axis(mut self) -> Self {
        let a = Point::origin();
        let b = a + Vector::unit_u();

        self.update_as_line_from_points([a, b]);
        self
    }

    fn update_as_v_axis(mut self) -> Self {
        let a = Point::origin();
        let b = a + Vector::unit_v();

        self.update_as_line_from_points([a, b]);
        self
    }

    fn update_as_circle_from_radius(
        &mut self,
        radius: impl Into<Scalar>,
    ) -> &mut Self {
        self.path = Some(SurfacePath::circle_from_radius(radius));
        self
    }

    fn update_as_line_from_points(
        &mut self,
        points: [impl Into<Point<2>>; 2],
    ) -> &mut Self {
        self.path = Some(SurfacePath::line_from_points(points));
        self
    }
}
