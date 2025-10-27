use fj_interop::Tolerance;
use fj_math::Point;

use crate::topology::face::HalfEdgeWithEndVertex;

/// # The approximation of a half-edge
///
/// The approximation of a half-edge is the approximation of its curve within
/// the boundary defined by the half-edge's start and end vertices, plus the
/// position of the start vertex.
///
/// By including the start vertex and not the end vertex, a whole chain of
/// half-edges can be approximated by simply appending the approximations of
/// each half-edge, without the necessity of any deduplication of points.
pub struct HalfEdgeApprox {
    pub points: Vec<Point<3>>,
}

impl HalfEdgeApprox {
    pub fn from_half_edge_with_end_vertex(
        HalfEdgeWithEndVertex {
            half_edge,
            end_vertex,
        }: HalfEdgeWithEndVertex,
        tolerance: Tolerance,
    ) -> Self {
        let [start, end] =
            [&half_edge.start, end_vertex].map(|vertex| vertex.point);

        let boundary_local = [start, end].map(|point_global| {
            half_edge.curve.geometry.project_point(point_global)
        });
        let points_local = half_edge
            .curve
            .geometry
            .approximate(boundary_local, tolerance);

        let points_global = points_local
            .points
            .into_iter()
            .map(|point| half_edge.curve.geometry.point_from_local(point.local))
            .collect();

        Self {
            points: points_global,
        }
    }
}
