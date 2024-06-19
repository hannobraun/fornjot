use fj_math::Point;

use super::{CurveBoundary, SurfacePath};

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

    /// Compute the surface position where the half-edge starts
    pub fn start_position(&self, path: &SurfacePath) -> Point<2> {
        // Computing the surface position from the curve position is fine.
        // `HalfEdge` "owns" its start position. There is no competing code that
        // could compute the surface position from slightly different data.

        let [start, _] = self.boundary.inner;
        path.point_from_path_coords(start)
    }
}
