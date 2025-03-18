use crate::geometry::TriMesh;

pub trait Object {
    fn to_tri_mesh(&self) -> TriMesh;
}
