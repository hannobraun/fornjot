use fj_interop::{Tolerance, TriMesh};

use crate::{geometry::ToTriMesh, handle::Handle};

use super::face::Face;

#[derive(Clone)]
pub struct Solid {
    faces: Vec<Handle<Face>>,
}

impl Solid {
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }
}

impl ToTriMesh for Solid {
    fn to_tri_mesh(&self, tolerance: impl Into<Tolerance>) -> TriMesh {
        let tolerance = tolerance.into();
        let mut tri_mesh = TriMesh::new();

        for face in &self.faces {
            tri_mesh = tri_mesh.merge(face.to_tri_mesh(tolerance));
        }

        tri_mesh
    }
}
