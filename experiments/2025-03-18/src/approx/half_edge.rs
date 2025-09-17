use fj_math::Point;

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
