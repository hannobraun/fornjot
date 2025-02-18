use std::fmt;

use crate::{
    geometry::TriMesh,
    operation::{Handle, HandleAny, Operation, OperationOutput},
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

impl Operation for Solid {
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

    fn children(&self) -> Vec<HandleAny> {
        self.faces.iter().map(|face| face.to_any()).collect()
    }
}
impl OperationOutput for Solid {
    fn output(&self) -> &Self {
        self
    }
}
