use fj_interop::{CircleApproxParams, Tolerance};
use fj_math::{Circle, Line, Point, Transform, Vector};

pub struct AbsoluteCurveGeometry {
    pub geometry: Box<dyn CurveGeometry>,
}

impl Clone for AbsoluteCurveGeometry {
    fn clone(&self) -> Self {
        Self {
            geometry: self.geometry.clone_curve_geometry(),
        }
    }
}

pub trait CurveGeometry {
    fn clone_curve_geometry(&self) -> Box<dyn CurveGeometry>;
    fn point_from_local(&self, point: Point<1>) -> Point<3>;
    fn project_point(&self, point: Point<3>) -> Point<1>;
    fn translate(&self, offset: Vector<3>) -> Box<dyn CurveGeometry>;

    /// # Approximate the curve
    ///
    /// Returns a list of points, in curve coordinates, that approximate the
    /// curve. The points must be within the provided boundary. Not outside of
    /// it, and not on it.
    ///
    /// ## Implementation Notes
    ///
    /// This method should take a tolerance parameter, to define how far the
    /// approximation is allowed to deviate from the actual curve. So far, this
    /// has not been necessary.
    fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> Vec<Point<1>>;
}

impl CurveGeometry for Circle<3> {
    fn clone_curve_geometry(&self) -> Box<dyn CurveGeometry> {
        Box::new(*self)
    }

    fn point_from_local(&self, point: Point<1>) -> Point<3> {
        self.point_from_circle_coords(point)
    }

    fn project_point(&self, point: Point<3>) -> Point<1> {
        self.point_to_circle_coords(point)
    }

    fn translate(&self, offset: Vector<3>) -> Box<dyn CurveGeometry> {
        let translated = self.transform(&Transform::translation(offset));
        Box::new(translated)
    }

    fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> Vec<Point<1>> {
        CircleApproxParams::new(self, tolerance)
            .approx_circle(boundary)
            .collect()
    }
}

impl CurveGeometry for Line<3> {
    fn clone_curve_geometry(&self) -> Box<dyn CurveGeometry> {
        Box::new(*self)
    }

    fn point_from_local(&self, point: Point<1>) -> Point<3> {
        self.point_from_line_coords(point)
    }

    fn project_point(&self, point: Point<3>) -> Point<1> {
        self.point_to_line_coords(point)
    }

    fn translate(&self, offset: Vector<3>) -> Box<dyn CurveGeometry> {
        let translated = self.transform(&Transform::translation(offset));
        Box::new(translated)
    }

    fn approximate(&self, _: [Point<1>; 2], _: Tolerance) -> Vec<Point<1>> {
        vec![]
    }
}
