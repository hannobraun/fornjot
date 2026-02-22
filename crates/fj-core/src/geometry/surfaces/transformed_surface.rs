use crate::{
    geometry::{Geometry, traits::GenTriMesh},
    interop::Tolerance,
    math::{Aabb, Point, Scalar, Transform, Triangle},
    storage::Handle,
    topology::Surface,
};

/// # A surface that is a transformation of another surface
pub struct TransformedSurface {
    /// # The original surface that is being transformed
    pub surface: Handle<Surface>,

    /// # The transform that is applied to the original surface
    pub transform: Transform,
}

impl GenTriMesh for TransformedSurface {
    fn origin(&self, geometry: &Geometry) -> Point<3> {
        let surface = geometry.of_surface_2(&self.surface).unwrap();
        self.transform
            .transform_point(&surface.generator.origin(geometry))
    }

    fn triangle_at(
        &self,
        point_surface: Point<2>,
        tolerance: Tolerance,
        geometry: &Geometry,
    ) -> (Triangle<3>, [Scalar; 3]) {
        let surface = geometry.of_surface_2(&self.surface).unwrap();
        let (triangle, barycentric_coords) =
            surface
                .generator
                .triangle_at(point_surface, tolerance, geometry);

        let triangle = self.transform.transform_triangle(&triangle);

        (triangle, barycentric_coords)
    }

    fn generate_tri_mesh(
        &self,
        boundary: Aabb<2>,
        tolerance: Tolerance,
        geometry: &Geometry,
    ) -> Vec<Point<2>> {
        // The triangle mesh is generated in 2D surface coordinates. No need to
        // transform that.
        let surface = geometry.of_surface_2(&self.surface).unwrap();
        surface
            .generator
            .generate_tri_mesh(boundary, tolerance, geometry)
    }
}
