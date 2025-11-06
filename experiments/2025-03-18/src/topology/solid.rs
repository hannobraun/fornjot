use fj_interop::{Tolerance, TriMesh};

use crate::handle::Handle;

use super::face::Face;

/// # A solid body
///
/// ## Implicit Duality Between Finite and Infinite Object
///
/// In 1D and 2D, there is a duality between infinite, mathematically defined
/// objects (`Curve` and `Surface`), and their finite counterparts, which are
/// defined as bounded regions on those infinite objects (`HalfEdge`, `Face`).
///
/// In a similar way, that same duality exists in 3D. `Solid` is the finite
/// object, defined as the bounded region of an infinite one. But that infinite
/// object is implicit. It is simply "all of the 3D space".
///
/// I think this falls naturally out of the fact that 3D is the highest
/// dimension we're dealing with here. If we supported four dimensions, then we
/// would need an infinite 3D object to describe 3D spaces in terms of the
/// implicit 4D space they'd be embedded in.
#[derive(Clone, Debug)]
pub struct Solid {
    pub faces: Vec<Handle<Face>>,
}

impl Solid {
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    pub fn to_tri_mesh(&self, tolerance: impl Into<Tolerance>) -> TriMesh {
        let tolerance = tolerance.into();
        let mut tri_mesh = TriMesh::new();

        for face in &self.faces {
            tri_mesh = tri_mesh.merge(face.to_tri_mesh(tolerance));
        }

        tri_mesh
    }
}
