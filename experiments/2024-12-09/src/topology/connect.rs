use std::fmt;

use crate::{
    geometry::{AnyOp, Handle, Operation, OperationOutput, TriMesh},
    math::Plane,
};

use super::{face::Face, solid::Solid};

pub trait ConnectExt {
    /// # Connect two faces by creating a side wall of faces from their vertices
    ///
    /// ## Panics
    ///
    /// Panics, if the two faces provided do not have the same number of
    /// vertices.
    ///
    /// ## Implementation Note
    ///
    /// This method has very particular (and undocumented) requirements about
    /// the orientation of the two faces relative to each other, and will
    /// happily generate invalid geometry, if those undocumented requirements
    /// aren't met.
    ///
    /// It should be seen as more of a placeholder for a real implementation of
    /// this operation.
    fn connect(self, other: Self) -> Connect;
}

impl ConnectExt for Handle<Face> {
    fn connect(self, other: Self) -> Connect {
        assert_eq!(
            self.vertices().count(),
            other.vertices().count(),
            "Can only connect faces that have the same number of vertices.",
        );

        let side_faces = self
            .half_edges()
            .zip(other.half_edges())
            .map(|([q, r], [t, s])| {
                let surface =
                    Plane::from_points([q, r, s].map(|vertex| vertex.point));
                let face = Face::new(
                    surface,
                    [q, r, s, t].map(|vertex| vertex.clone()),
                );
                Handle::new(face)
            })
            .collect::<Vec<_>>();

        let output = Solid::new([self, other].into_iter().chain(side_faces));
        Connect {
            output: Handle::new(output),
        }
    }
}

pub struct Connect {
    pub output: Handle<Solid>,
}

impl Operation for Connect {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connect")
    }

    fn tri_mesh(&self) -> TriMesh {
        self.output.tri_mesh()
    }

    fn children(&self) -> Vec<AnyOp> {
        vec![self.output.to_any()]
    }
}

impl OperationOutput<Solid> for Connect {
    fn output(&self) -> &Solid {
        &self.output
    }
}
