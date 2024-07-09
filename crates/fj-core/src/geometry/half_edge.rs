use fj_math::Point;

use super::CurveBoundary;

/// The geometry of a half-edge
#[derive(Copy, Clone, Debug)]
pub struct HalfEdgeGeom {
    /// # The boundary of the half-edge on its curve
    pub boundary: CurveBoundary<Point<1>>,
}

impl HalfEdgeGeom {
    /// Update the boundary
    pub fn with_boundary(
        mut self,
        boundary: impl Into<CurveBoundary<Point<1>>>,
    ) -> Self {
        self.boundary = boundary.into();
        self
    }
}
