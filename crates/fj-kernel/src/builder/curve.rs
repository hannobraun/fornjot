use fj_math::{Point, Scalar};

use crate::{geometry::path::SurfacePath, partial::PartialCurve};

/// Builder API for [`PartialCurve`]
pub trait CurveBuilder {
    /// Update partial curve to be a circle, from the provided radius
    ///
    /// Returns the updated path.
    fn update_as_circle_from_radius(
        &mut self,
        radius: impl Into<Scalar>,
    ) -> SurfacePath;

    /// Update partial curve to be a circle, from the provided radius
    ///
    /// Returns the updated path.
    fn update_as_circle_from_center_and_radius(
        &mut self,
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
    ) -> SurfacePath;

    /// Update partial curve to be a line, from the provided points
    ///
    /// Returns the updated path.
    fn update_as_line_from_points(
        &mut self,
        points: [impl Into<Point<2>>; 2],
    ) -> SurfacePath;

    /// Update partial curve to be a line, from provided points and line coords
    ///
    /// Returns the updated path.
    fn update_as_line_from_points_with_line_coords(
        &mut self,
        points: [(impl Into<Point<1>>, impl Into<Point<2>>); 2],
    ) -> SurfacePath;
}

impl CurveBuilder for PartialCurve {
    fn update_as_circle_from_radius(
        &mut self,
        radius: impl Into<Scalar>,
    ) -> SurfacePath {
        let path = SurfacePath::circle_from_radius(radius);
        self.path = Some(path.into());
        path
    }

    fn update_as_circle_from_center_and_radius(
        &mut self,
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
    ) -> SurfacePath {
        let path = SurfacePath::circle_from_center_and_radius(center, radius);
        self.path = Some(path.into());
        path
    }

    fn update_as_line_from_points(
        &mut self,
        points: [impl Into<Point<2>>; 2],
    ) -> SurfacePath {
        let (path, _) = SurfacePath::line_from_points(points);
        self.path = Some(path.into());
        path
    }

    fn update_as_line_from_points_with_line_coords(
        &mut self,
        points: [(impl Into<Point<1>>, impl Into<Point<2>>); 2],
    ) -> SurfacePath {
        let path = SurfacePath::from_points_with_line_coords(points);
        self.path = Some(path.into());
        path
    }
}
