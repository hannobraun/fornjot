use fj_math::Point;

use super::{CurveBoundary, SurfacePath};

/// The geometry of a half-edge
#[derive(Copy, Clone, Debug)]
pub struct HalfEdgeGeom {
    /// # The path of the half-edge
    ///
    /// ## Implementation Note
    ///
    /// Currently, all curve-related geometry is defined locally, in terms of
    /// the surface that the curve is on (or purely in 2D, if there is no
    /// surface associated with this geometry). However, curves exist globally,
    /// independently of surfaces. Half-edges in multiple surfaces can refer to
    /// the same curve, and in fact, that is the whole reason for their
    /// existence as a topological object.
    ///
    /// This contradiction, globally defined curves but locally defined curve
    /// geometry, is the reason that this curve geometry is defined right here,
    /// associated with a locally existing half-edge. (And, I might add,
    /// redundantly so, as multiple half-edges within the same surface context
    /// can refer to the same curve.)
    ///
    /// Instead, it should be possible to define curve geometry *either* locally
    /// or globally. Then that respective definition can be associated with the
    /// curve (and possibly, in addition, a surface). How exactly that is going
    /// to work is up in the air.
    ///
    /// The point of all this exposition is to clarify that this field doesn't
    /// really belong here. It exists here for practical reasons that are,
    /// hopefully, temporary.
    pub path: SurfacePath,

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
