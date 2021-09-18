use crate::Mesh;

/// Implemented for geometry that can be converted to a triangle mesh
pub trait TriangleMesh {
    /// Convert geometry to a triangle mesh
    fn mesh(&self) -> Mesh;
}
