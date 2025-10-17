use fj_interop::Tolerance;

use crate::approx::{
    half_edge::HalfEdgeApprox, point::ApproxPoint, surface::SurfaceApprox,
};

pub struct FaceApproxPoints {
    pub points: Vec<ApproxPoint<2>>,
}

impl FaceApproxPoints {
    pub fn from_half_edge_approx(
        half_edges: impl IntoIterator<Item = HalfEdgeApprox>,
        surface: &mut SurfaceApprox,
        tolerance: impl Into<Tolerance>,
    ) -> Self {
        let tolerance = tolerance.into();

        let points = half_edges
            .into_iter()
            .flat_map(|approx| approx.points)
            .map(|point_global| {
                // Here, we project a 3D point (from the vertex) into the face's
                // surface, creating a 2D point. Through the surface, this 2D
                // point has a position in 3D space.
                //
                // But this position isn't necessarily going to be the same as
                // the position of the original 3D point, due to numerical
                // inaccuracy.
                //
                // This doesn't matter. Neither does the fact, that other faces
                // might share the same vertices and project them into their own
                // surfaces, creating more redundancy.
                //
                // The reason that it doesn't, is that we're using the projected
                // 2D points _only_ for this local triangulation. Once that
                // tells us how the different 3D points must connect, we use the
                // original 3D points to build those triangles. We never convert
                // the 2D points back into 3D.
                let point_surface =
                    surface.project_point(point_global, tolerance.inner());

                ApproxPoint {
                    local: point_surface,
                    global: point_global,
                }
            })
            .collect();

        Self { points }
    }
}
