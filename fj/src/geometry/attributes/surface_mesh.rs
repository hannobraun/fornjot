use crate::geometry::shapes::Mesh;

/// Compute a triangle mesh that approximates a shape's surface
pub trait SurfaceMesh<const D: usize> {
    /// Compute surface mesh for shape
    fn mesh(&self) -> Mesh<D>;
}
