use fj_math::{Circle, Point};

use crate::geometry::{
    curves::circle::CircleApproxParams, CurveBoundary, Tolerance,
};

/// # Approximate a circle
///
/// ## Arguments
///
/// Besides a circle, this method takes two arguments:
///
/// - The `boundary` within which the circle should be approximated.
/// - The `tolerance` that specifies how much the approximation is allowed to
///   deviate from the actual circle.
///
/// ## Return Value
///
/// The approximation returns points within the provided boundary. The boundary
/// points themselves are not included in the approximation. This gives the
/// caller (who knows the boundary anyway) more options for how to further
/// process the approximation.
///
/// ## Determinism
///
/// Circle approximation is carefully designed to produce a deterministic result
/// for the combination of a given circle and tolerance, regardless of the
/// boundary. This is done to prevent invalid meshes from being generated.
///
/// In specific terms, this means there is an infinite set of points that
/// approximates a circle (infinite, since the circle's local coordinate space
/// is infinite). That set is deterministic for a given combination of circle
/// and tolerance. The boundary that defines where the circle is approximated
/// only influences the result in two ways:
///
/// 1. It controls which points from the infinite set are actually computed.
/// 2. It defines the order in which the computed points are returned.
///
/// As a result, circle approximation is guaranteed to generate points that can
/// fit together in a valid mesh, no matter which ranges of a path are being
/// approximated, and how many times.
pub fn approx_circle<const D: usize>(
    circle: &Circle<D>,
    boundary: impl Into<CurveBoundary<Point<1>>>,
    tolerance: impl Into<Tolerance>,
) -> Vec<(Point<1>, Point<D>)> {
    let boundary = boundary.into();
    let tolerance = tolerance.into();

    let params = CircleApproxParams::new(circle, tolerance);
    let mut points = Vec::new();

    for point_curve in params.approx_circle(boundary) {
        let point_global = circle.point_from_circle_coords(point_curve);
        points.push((point_curve, point_global));
    }

    points
}
