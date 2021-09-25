use crate::geometry::shapes::Mesh;

/// Implemented for geometry that can be converted to a triangle mesh
pub trait SurfaceMesh<const D: usize> {
    /// Convert geometry to a triangle mesh
    fn mesh(&self) -> Mesh<D>;
}
