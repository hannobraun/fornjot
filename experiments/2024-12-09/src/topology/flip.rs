use std::fmt;

use crate::geometry::{AnyOp, Handle, Operation, TriMesh};

use super::face::Face;

pub trait FlipExt {
    fn flip(self) -> Flip;
}

impl FlipExt for Handle<Face> {
    fn flip(self) -> Flip {
        let output = Handle::new(Face::new(
            self.surface().flip(),
            self.vertices().cloned(),
        ));

        Flip { output }
    }
}

pub struct Flip {
    pub output: Handle<Face>,
}

impl Operation for Flip {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Flip")
    }

    fn tri_mesh(&self) -> TriMesh {
        self.output.tri_mesh()
    }

    fn children(&self) -> Vec<AnyOp> {
        vec![self.output.to_any()]
    }
}
