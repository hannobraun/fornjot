use crate::geometry::TriMesh;

pub trait ToTriMesh {
    fn to_tri_mesh(&self) -> TriMesh;
}
