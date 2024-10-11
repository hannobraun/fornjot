use std::sync::Arc;

use fj_math::{Aabb, Point, Scalar, Transform, Triangle};

use crate::geometry::{traits::GenTriMesh, Tolerance};

/// # A surface that is a transformation of another surface
pub struct TransformedSurface {
    /// # The original surface that is being transformed
    pub surface: Arc<dyn GenTriMesh>,

    /// # The transform that is applied to the original surface
    pub transform: Transform,
}

impl GenTriMesh for TransformedSurface {
    fn origin(&self) -> Point<3> {
        self.transform.transform_point(&self.surface.origin())
    }

    fn triangle_at(
        &self,
        point_surface: Point<2>,
        tolerance: Tolerance,
    ) -> (Triangle<3>, [Scalar; 3]) {
        let (triangle, barycentric_coords) =
            self.surface.triangle_at(point_surface, tolerance);

        let triangle = self.transform.transform_triangle(&triangle);

        (triangle, barycentric_coords)
    }

    fn generate_tri_mesh(
        &self,
        boundary: Aabb<2>,
        tolerance: Tolerance,
    ) -> Vec<Point<2>> {
        // The triangle mesh is generated in 2D surface coordinates. No need to
        // transform that.
        self.surface.generate_tri_mesh(boundary, tolerance)
    }
}
