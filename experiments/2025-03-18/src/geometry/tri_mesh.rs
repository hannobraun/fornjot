use fj_interop::{Tolerance, TriMesh};

pub trait ToTriMesh {
    fn to_tri_mesh(&self, tolerance: impl Into<Tolerance>) -> TriMesh;
}
