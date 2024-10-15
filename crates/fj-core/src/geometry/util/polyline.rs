//! # Geometric utility code based on polylines

use fj_math::{Aabb, Point};

use crate::geometry::{traits::GenPolyline, CurveBoundary, Tolerance};

/// # A polyline, the uniform representation of curve geometry
///
/// Can be 2- or 3-dimensional, as specified by the `D` type parameter.
pub struct Polyline<const D: usize> {
    /// # The points that make up the vertices between the line segments
    pub points: Vec<Point<D>>,

    /// # The same points as the ones in the `points` field, but in curve coords
    pub points_curve: Vec<Point<1>>,
}

/// # Convert a point on a curve from curve coordinates to surface coordinates
pub fn curve_point_to_surface_point(
    curve: &dyn GenPolyline<2>,
    point_curve: impl Into<Point<1>>,
    tolerance: impl Into<Tolerance>,
) -> Point<2> {
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
        curve_point_to_surface_point(curve, point_curve, tolerance)
    });

    Aabb::<2>::from_points(points_surface)
}
