use crate::geometry::{
    attributes::Edges,
    shapes::{Mesh, Toroid},
};

/// Compute a triangle mesh that approximates a shape's surface
pub trait SurfaceMesh<const D: usize> {
    /// Compute surface mesh for shape
    ///
    /// If the surface mesh can only be approximated, `n` defines the number of
    /// computational steps taken to compute the mesh.
    // TASK: Replace `n` with a tolerance value.
    fn surface_mesh(&self, n: u32) -> Mesh<D>;
}

impl<T> SurfaceMesh<3> for Toroid<T>
where
    T: Edges<2>,
{
    fn surface_mesh(&self, _n: u32) -> Mesh<3> {
        // TASK: Implement.
        todo!()
    }
}
