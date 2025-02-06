use std::fmt;

use crate::geometry::{AnyOp, Handle, Operation, TriMesh};

use super::face::Face;

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

impl Operation for Solid {
    type Output = Self;

    fn output(&self) -> &Self::Output {
        self
    }

    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Solid")
    }

    fn tri_mesh(&self) -> TriMesh {
        let mut tri_mesh = TriMesh::new();

        for face in &self.faces {
            tri_mesh = tri_mesh.merge(face.tri_mesh());
        }

        tri_mesh
    }

    fn children(&self) -> Vec<AnyOp> {
        self.faces.iter().map(|face| face.to_any()).collect()
    }
}
