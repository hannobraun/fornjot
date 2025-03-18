use crate::{
    geometry::TriMesh,
    object::{Handle, Object},
};

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

impl Object for Solid {
    fn tri_mesh(&self) -> TriMesh {
        let mut tri_mesh = TriMesh::new();

        for face in &self.faces {
            tri_mesh = tri_mesh.merge(face.tri_mesh());
        }

        tri_mesh
    }
}
