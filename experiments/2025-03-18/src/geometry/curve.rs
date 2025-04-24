use fj_interop::{CircleApproxParams, Tolerance};
use fj_math::{Circle, Line, Point, Transform, Vector};

/// # Curve geometry that has a fixed position (is _anchored_) in space
///
/// The opposite would be _floating_ curve geometry, which could be relative to
/// any point.
///
/// In terms of a line, for example, the anchored version is the full line, an
/// origin and a direction (a point and a vector). The floating version is just
/// the direction (a vector).
pub struct AnchoredCurveGeometry {
    /// # The origin point of the curve, which anchors it in 3D space
    ///
    /// This _must always_ be the origin point of the curve's coordinate system.
    /// Using something like the center of a circle is not valid!
    pub origin: Point<3>,

    /// # The floating part of the curve geometry
    pub geometry: Box<dyn CurveGeometry>,
}

impl AnchoredCurveGeometry {
    pub fn point_from_local(&self, point: Point<1>) -> Point<3> {
        self.geometry.point_from_local(point)
    }

    pub fn project_point(&self, point: Point<3>) -> Point<1> {
        self.geometry.project_point(point)
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();

        Self {
            origin: Transform::translation(offset)
                .transform_point(&self.origin),
            geometry: self.geometry.translate(offset),
        }
    }

    pub fn approximate(
        &self,
        boundary: [Point<1>; 2],
        tolerance: Tolerance,
    ) -> Vec<Point<1>> {
        self.geometry.approximate(boundary, tolerance)
    }
}

impl Clone for AnchoredCurveGeometry {
    fn clone(&self) -> Self {
        Self {
            origin: self.origin,
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
