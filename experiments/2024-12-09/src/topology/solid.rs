use std::fmt;

use crate::{
    geometry::TriMesh,
    object::{Handle, HandleAny, Object},
};

use super::face::Face;

/// # A solid
///
/// Solids are 3D objects that are bounded by faces.
#[derive(Clone)]
pub struct Solid {
    faces: Vec<Handle<Face>>,
}

impl Solid {
    /// # Create a solid from its component parts
    ///
    /// Check out [`operations`](crate::operations) for more interesting ways to
    /// create solids.
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }
}

impl Object for Solid {
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
