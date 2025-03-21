use fj_interop::TriMesh;

pub trait ToTriMesh {
    fn to_tri_mesh(&self) -> TriMesh;
}
