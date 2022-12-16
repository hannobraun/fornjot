use fj_math::{Point, Scalar, Vector};

use crate::{geometry::path::SurfacePath, partial::PartialCurve};

/// Builder API for [`PartialCurve`]
pub trait CurveBuilder {
    /// Update partial curve to represent the u-axis of the surface it is on
    fn update_as_u_axis(&mut self);

    /// Update partial curve to represent the v-axis of the surface it is on
    fn update_as_v_axis(&mut self);

    /// Update partial curve to be a circle, from the provided radius
    fn update_as_circle_from_radius(&mut self, radius: impl Into<Scalar>);

    /// Update partial curve to be a line, from the provided points
    fn update_as_line_from_points(&mut self, points: [impl Into<Point<2>>; 2]);
}

impl CurveBuilder for PartialCurve {
    fn update_as_u_axis(&mut self) {
        let a = Point::origin();
        let b = a + Vector::unit_u();

        self.update_as_line_from_points([a, b])
    }

    fn update_as_v_axis(&mut self) {
        let a = Point::origin();
        let b = a + Vector::unit_v();

        self.update_as_line_from_points([a, b])
    }

    fn update_as_circle_from_radius(&mut self, radius: impl Into<Scalar>) {
        self.path = Some(SurfacePath::circle_from_radius(radius));
    }

    fn update_as_line_from_points(&mut self, points: [impl Into<Point<2>>; 2]) {
        let path = SurfacePath::line_from_points(points);
        self.path = Some(path);
    }
}
