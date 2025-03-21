pub use fj_interop::MeshTriangle;

use fj_math::Triangle;

#[derive(Debug)]
pub struct TriMesh {
    pub triangles: Vec<MeshTriangle>,
}

impl TriMesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.triangles.extend(other.triangles);
        self
    }

    pub fn all_triangles(&self) -> impl Iterator<Item = Triangle<3>> {
        self.triangles.iter().map(|triangle| triangle.inner)
    }

    pub fn external_triangles(&self) -> impl Iterator<Item = Triangle<3>> {
        self.triangles.iter().filter_map(|triangle| {
            (!triangle.is_internal).then_some(triangle.inner)
        })
    }
}

pub trait ToTriMesh {
    fn to_tri_mesh(&self) -> TriMesh;
}
