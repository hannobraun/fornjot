use crate::geometry::TriMesh;

pub trait Object {
    fn tri_mesh(&self) -> TriMesh;
}
