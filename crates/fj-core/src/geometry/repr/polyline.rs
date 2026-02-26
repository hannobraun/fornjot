//! # Geometric utility code based on polylines

use std::iter;

use crate::{
    approx::Tolerance, geometry::{CurveBoundary, traits::GenPolyline}, math::{Aabb, LineSegment, Point}
};

/// # A polyline, the uniform representation of curve geometry
///
/// Can be 2- or 3-dimensional, as specified by the `D` type parameter.
pub struct Polyline<const D: usize> {
    /// # The connecting points between line segments
    pub points: Vec<Point<D>>,

    /// # The connecting points between line segments, in curve coordinates
    pub points_curve: Vec<Point<1>>,
}

impl<const D: usize> Polyline<D> {
    /// # Create an instance of `Polyline` from a curve
    pub fn from_curve(
        curve: &dyn GenPolyline<D>,
        boundary: impl Into<CurveBoundary<Point<1>>>,
        tolerance: impl Into<Tolerance>,
    ) -> Self {
        let boundary = boundary.into();
        let tolerance = tolerance.into();

        let points_curve = curve.generate_polyline(boundary, tolerance);
        let points = points_curve
            .iter()
            .map(|&point_curve| {
                convert_from_curve_point(curve, point_curve, tolerance)
            })
            .collect();

        Self {
            points,
            points_curve,
        }
    }

    /// # Iterate over the line segments of this polyline
    pub fn line_segments(&self) -> impl Iterator<Item = LineSegment<D>> + '_ {
        let mut i = 0;

        iter::from_fn(move || {
            let points = [*self.points.get(i)?, *self.points.get(i + 1)?];
            let points_line =
                [*self.points_curve.get(i)?, *self.points_curve.get(i + 1)?];

            i += 1;

            Some(LineSegment {
                points,
                points_line,
            })
        })
    }
}

/// # Convert a point on a curve from curve coordinates to surface coordinates
pub fn convert_from_curve_point<const D: usize>(
    curve: &dyn GenPolyline<D>,
    point_curve: impl Into<Point<1>>,
    tolerance: impl Into<Tolerance>,
) -> Point<D> {
    let point_curve = point_curve.into();
    let tolerance = tolerance.into();

    let line_segment = curve.line_segment_at(point_curve, tolerance);
    let line = line_segment.to_line();

    line.point_from_line_coords(point_curve)
}

/// # Generate a 2D axis-aligned bounding box for a curve in a given range
pub fn surface_aabb_from_bounded_curve(
    curve: &dyn GenPolyline<2>,
    boundary: impl Into<CurveBoundary<Point<1>>>,
    tolerance: impl Into<Tolerance>,
) -> Aabb<2> {
    let boundary = boundary.into();
    let tolerance = tolerance.into();

    let points_curve = curve.generate_polyline(boundary, tolerance);
    let points_surface = points_curve.into_iter().map(|point_curve| {
        convert_from_curve_point(curve, point_curve, tolerance)
    });

    Aabb::<2>::from_points(points_surface)
}
